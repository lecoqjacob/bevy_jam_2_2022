use crate::round::*;
use bevy::{math::Vec3Swizzles, tasks::ComputeTaskPool};
use parking_lot::Mutex;
use std::sync::Arc;

pub mod creature_settings {
    pub const FOLLOW_PLAYER_MIN_DISTANCE: f32 = 25.;
    pub const FOLLOW_PLAYER_MAX_DISTANCE: f32 = 75.;

    pub const TARGET_PLAYER_MIN_DISTANCE: f32 = 15.;
    pub const TARGET_PLAYER_MAX_DISTANCE: f32 = 30.;

    pub const TARGET_COLLECTION_DISTANCE: f32 = 100.;

    pub const CREATURE_SPEED: f32 = 300.;
    pub const CREATURE_VISION: f32 = 110.;
    pub const DEFAULT_CREATURE_SIZE: (f32, f32) = (10., 15.); // (min, max)

    pub const CREATURE_COLLISION_AVOIDANCE: f32 = 4.;
    pub const CREATURE_COHESION: f32 = 5.;
    pub const CREATURE_SEPERATION: f32 = 3.;
    pub const CREATURE_ALIGNMENT: f32 = 15.;
    pub const CREATURE_CHASE: f32 = 15.;
}

pub fn spawn_zombie(
    commands: &mut Commands,
    rip: &mut RollbackIdProvider,
    transform: Transform,
    direction_vector: Vec2,
    size: f32,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            transform,
            sprite: Sprite {
                color: Color::SEA_GREEN,
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            ..default()
        })
        .insert(Direction(direction_vector))
        .insert(CreatureType::default())
        .insert(CreatureSize(size))
        .insert(Rollback::new(rip.next_id()))
        .insert(RoundEntity)
        .id()
}

pub fn apply_force_event_system(
    mut apply_force_event_handler: EventReader<ApplyForceEvent>,
    mut creature_query: Query<&mut crate::components::Direction>,
) {
    for ApplyForceEvent(entity, force, factor) in apply_force_event_handler.iter() {
        if let Ok(mut direction) = creature_query.get_mut(*entity) {
            if direction.0.is_nan() {
                continue;
            }

            direction.lerp(*force, factor * TIME_STEP);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Creature Movement Systems
////////////////////////////////////////////////////////////////////////////////

pub fn creatures_follow(
    map_settings: Res<MapSettings>,
    player_q: Query<(Entity, &Transform), (With<Player>, Without<CreatureType>)>,
    mut creatures: Query<(
        &mut Transform,
        &crate::components::Direction,
        &CreatureType,
        &CreatureFollow,
    )>,
) {
    for (mut transform, direction, c_type, c_follow) in &mut creatures {
        if let Some(player_transform) =
            player_q.iter().find(|(p, _)| *p == c_type.0.unwrap()).map(|(_, t)| t)
        {
            let player_translation = player_transform.translation.xy();

            let distance = player_translation.distance(transform.translation.xy());
            let speed =
                if distance < c_follow.0 { continue } else { creature_settings::CREATURE_SPEED };

            // Move and rotate based on direction
            move_target(&mut transform, direction, speed, &map_settings);
        }
    }
}

pub fn creatures_target(
    map_settings: Res<MapSettings>,
    mut query: Query<
        (&mut Transform, &crate::components::Direction),
        (With<CreatureTarget>, Without<CreatureFollow>),
    >,
) {
    for (mut transform, direction) in &mut query {
        move_target(&mut transform, direction, creature_settings::CREATURE_SPEED, &map_settings);
    }
}

fn move_target(
    transform: &mut Transform,
    direction: &crate::components::Direction,
    speed: f32,
    map_settings: &MapSettings,
) {
    // Move and rotate based on direction
    transform.translation.x += direction.0.x * speed * TIME_STEP;
    transform.translation.y += direction.0.y * speed * TIME_STEP;
    transform.rotation = Quat::from_rotation_z(-direction.0.x.atan2(direction.0.y));

    // Clamp to map bounds
    let (map_width, map_height) = (map_settings.width, map_settings.height);
    transform.translation.x = transform.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
    transform.translation.y = transform.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
}

////////////////////////////////////////////////////////////////////////////////
// Creature Utility Systems
////////////////////////////////////////////////////////////////////////////////

pub fn creature_grow(mut creatures: Query<(&mut Sprite, &CreatureSize)>) {
    for (mut c_sprite, c_size) in &mut creatures {
        let mut sprite_size = c_sprite.custom_size.unwrap();
        let new_size = Vec2::new(c_size.0, c_size.0);
        if sprite_size == new_size {
            continue;
        }

        let diff = (sprite_size - new_size).abs();
        if diff.x <= 1. || diff.y <= 1. {
            c_sprite.custom_size = Some(new_size);
            continue;
        }

        sprite_size = sprite_size.lerp(new_size, 2. * TIME_STEP);
        c_sprite.custom_size = Some(sprite_size);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Boid Systems (Flocking & Chasing)
////////////////////////////////////////////////////////////////////////////////

pub fn flocking_system(
    cache_grid: Res<CacheGrid>,
    apply_force_event_handler: EventWriter<ApplyForceEvent>,
    creatures: Query<(
        Entity,
        &crate::components::Direction,
        &Transform,
        &CreatureType,
        &CreatureSize,
    )>,
) {
    let creature_vec = creatures.iter().collect::<Vec<_>>();
    let compute_task_pool = ComputeTaskPool::get();
    let creatures_per_thread = creature_vec.len() / compute_task_pool.thread_num();
    if creatures_per_thread == 0 {
        return;
    }

    let creatures = &creatures;
    let cache_grid = &cache_grid;
    let apply_force_event_handler = Arc::new(Mutex::new(apply_force_event_handler));

    compute_task_pool.scope(|scope| {
        for chunk in creature_vec.chunks(creatures_per_thread) {
            let apply_force_event_handler = apply_force_event_handler.clone();

            scope.spawn(async move {
                for (entity_a, _, transform_a, type_a, size) in chunk {
                    let entity_a = *entity_a;
                    let type_a = *type_a;
                    let position_a = transform_a.translation.xy();

                    let mut average_position = Vec2::ZERO; // Cohesion
                    let mut average_direction = Vec2::ZERO; // Alignment
                    let mut average_close_position = Vec2::ZERO; // Separation

                    let mut vision_count = 0;
                    let mut half_vision_count = 0;

                    let size = Vec2::new(size.0, size.0);

                    let (collision_avoidance, cohesion, separation, alignment) = (
                        creature_settings::CREATURE_COLLISION_AVOIDANCE,
                        creature_settings::CREATURE_COHESION,
                        creature_settings::CREATURE_SEPERATION,
                        creature_settings::CREATURE_ALIGNMENT,
                    );

                    for entity_b in cache_grid
                        .get_nearby_entities(position_a, creature_settings::CREATURE_VISION)
                        .iter()
                        .filter(|e| **e != entity_a)
                    {
                        let (_, direction_b, transform_b, type_b, _) =
                            match creatures.get(*entity_b) {
                                Ok(c) => c,
                                Err(_) => continue,
                            };

                        // Only flock with similar creatures
                        if type_a != type_b {
                            continue;
                        }

                        let position_b = transform_b.translation.xy();
                        let distance = position_a.distance(position_b);
                        if distance <= creature_settings::CREATURE_VISION {
                            vision_count += 1;
                            average_position += position_b;
                            average_direction += direction_b.0;
                        }
                        if distance <= creature_settings::CREATURE_VISION / 2.0 {
                            half_vision_count += 1;
                            average_close_position += position_b;
                        }

                        if distance <= size.max_element() * 2.0 {
                            let away_direction = (position_a - position_b).normalize();
                            apply_force_event_handler.lock().send(ApplyForceEvent(
                                entity_a,
                                away_direction,
                                collision_avoidance,
                            ));
                        }
                    }

                    if vision_count > 0 {
                        average_position /= vision_count as f32;
                        average_direction /= vision_count as f32;
                        let cohesion_force =
                            (average_position - transform_a.translation.xy()).normalize();
                        apply_force_event_handler.lock().send(ApplyForceEvent(
                            entity_a,
                            cohesion_force,
                            cohesion,
                        ));
                        apply_force_event_handler.lock().send(ApplyForceEvent(
                            entity_a,
                            average_direction.normalize(),
                            alignment,
                        ));
                    }

                    if half_vision_count > 0 {
                        average_close_position /= half_vision_count as f32;
                        let separation_force = (position_a - average_close_position).normalize();
                        apply_force_event_handler.lock().send(ApplyForceEvent(
                            entity_a,
                            separation_force,
                            separation,
                        ));
                    }
                }
            });
        }
    });
}

pub fn follow_system(
    players: Query<&Transform, With<Player>>,
    apply_force_event_handler: EventWriter<ApplyForceEvent>,
    creatures: Query<
        (Entity, &Transform, &CreatureType, Option<&CreatureFollow>, Option<&CreatureTarget>),
        Without<Player>,
    >,
) {
    let creature_vec = creatures.iter().collect::<Vec<_>>();
    let compute_task_pool = ComputeTaskPool::get();
    let creatures_per_thread = usize::max(1, creature_vec.len() / compute_task_pool.thread_num());
    if creatures_per_thread == 0 {
        return;
    }

    let players = &players;
    let apply_force_event_handler = Arc::new(Mutex::new(apply_force_event_handler));

    compute_task_pool.scope(|scope| {
        for chunk in creature_vec.chunks(creatures_per_thread) {
            let apply_force_event_handler = apply_force_event_handler.clone();
            scope.spawn(async move {
                for (entity, transform, c_type, c_follow, c_target) in chunk {
                    let position_a = transform.translation.xy();

                    let (target, dist) = match (c_follow, c_target) {
                        (Some(f), None) => (c_type.0.unwrap(), f.0),
                        (None, Some(t)) => (t.0, 1.0),
                        _ => continue,
                    };

                    let player_position = match players.get(target) {
                        Ok(player_transform) => player_transform.translation.xy(),
                        Err(_) => continue,
                    };

                    let distance = position_a.distance(player_position);
                    if distance > dist {
                        let chase_direction = (player_position - position_a).normalize();
                        apply_force_event_handler.lock().send(ApplyForceEvent(
                            *entity,
                            chase_direction,
                            creature_settings::CREATURE_CHASE,
                        ));
                    }
                }
            });
        }
    });
}
