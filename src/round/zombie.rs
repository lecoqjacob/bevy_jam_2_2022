use crate::round::*;
use bevy::{math::Vec3Swizzles, tasks::ComputeTaskPool};
use parking_lot::Mutex;
use std::sync::Arc;

pub mod zombie_settings {
    pub const FOLLOW_PLAYER_MIN_DISTANCE: f32 = 25.;
    pub const FOLLOW_PLAYER_MAX_DISTANCE: f32 = 75.;

    pub const TARGET_PLAYER_MIN_DISTANCE: f32 = 15.;
    pub const TARGET_PLAYER_MAX_DISTANCE: f32 = 30.;

    pub const TARGET_COLLECTION_DISTANCE: f32 = 100.;

    pub const ZOMBIE_SPEED: f32 = 210.;
    pub const ZOMBIE_VISION: f32 = 110.;
    pub const DEFAULT_ZOMBIE_SIZE: (f32, f32) = (10., 15.); // (min, max)

    pub const ZOMBIE_COLLISION_AVOIDANCE: f32 = 4.;
    pub const ZOMBIE_COHESION: f32 = 5.;
    pub const ZOMBIE_SEPERATION: f32 = 3.;
    pub const ZOMBIE_ALIGNMENT: f32 = 15.;
    pub const ZOMBIE_CHASE: f32 = 15.;

    pub const ZOMBIE_ATTACK_COOLDOWN: f32 = 1.;
}

///////////////////////////////////////////////////////////////////////////////
// Zombie Components
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////

pub fn spawn_zombie(
    commands: &mut Commands,
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
        .insert(Health(2))
        .insert(RoundEntity)
        .insert(Clock::new(1.))
        .id()
}

pub fn apply_force_event_system(
    time: Res<Time>,
    mut apply_force_event_handler: EventReader<ApplyForceEvent>,
    mut zombie_query: Query<&mut crate::components::Direction>,
) {
    for ApplyForceEvent(entity, force, factor) in apply_force_event_handler.iter() {
        if let Ok(mut direction) = zombie_query.get_mut(*entity) {
            if direction.0.is_nan() {
                continue;
            }
            direction.lerp(*force, factor * time.delta_seconds());
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Creature Movement Systems
////////////////////////////////////////////////////////////////////////////////

pub fn creatures_follow(
    time: Res<Time>,
    map_settings: Res<MapSettings>,
    player_q: Query<(Entity, &Transform), (With<Player>, Without<CreatureType>)>,
    mut creatures: Query<
        (&mut Transform, &crate::components::Direction, &CreatureType, &CreatureFollow),
        Without<CreatureTarget>,
    >,
) {
    for (mut transform, direction, c_type, c_follow) in &mut creatures {
        if let Some(player_transform) =
            player_q.iter().find(|(p, _)| *p == c_type.0.unwrap()).map(|(_, t)| t)
        {
            let player_translation = player_transform.translation.xy();

            let distance = player_translation.distance(transform.translation.xy());
            let speed =
                if distance < c_follow.0 { continue } else { zombie_settings::ZOMBIE_SPEED };

            // Move and rotate based on direction
            move_target(&time, &mut transform, direction, speed, &map_settings);
        }
    }
}

pub fn creatures_target(
    time: Res<Time>,
    mut commands: Commands,
    map_settings: Res<MapSettings>,
    mut damage_events: EventWriter<DamageEvent>,
    mut player_q: Query<(Entity, &Transform, &Player), (With<Player>, Without<CreatureType>)>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &crate::components::Direction,
            &CreatureTarget,
            &CreatureSize,
            &mut Clock,
        ),
        (With<CreatureTarget>, Without<Player>),
    >,
) {
    for (creature, mut transform, direction, target, size, mut clock) in &mut query {
        if let Some((p_entity, p_transform, player)) =
            player_q.iter_mut().find(|(p, _, _)| *p == target.0)
        {
            let player_translation = p_transform.translation.xy();
            let distance = player_translation.distance(transform.translation.xy());

            let attack_distance = size.0 + player.size;
            if distance < attack_distance {
                clock.current = f32::max(clock.current - time.delta_seconds(), 0.0);
                if clock.current <= 0.0 {
                    clock.reset();
                    damage_events.send(DamageEvent::new(p_entity, creature));
                }
            } else {
                move_target(
                    &time,
                    &mut transform,
                    direction,
                    zombie_settings::ZOMBIE_SPEED,
                    &map_settings,
                );
            };
        } else {
            commands.entity(creature).remove::<CreatureTarget>();
        }
    }
}

fn move_target(
    time: &Time,
    transform: &mut Transform,
    direction: &crate::components::Direction,
    speed: f32,
    map_settings: &MapSettings,
) {
    // Move and rotate based on direction
    transform.translation.x += direction.0.x * speed * time.delta_seconds();
    transform.translation.y += direction.0.y * speed * time.delta_seconds();
    transform.rotation = Quat::from_rotation_z(-direction.0.x.atan2(direction.0.y));

    // Clamp to map bounds
    let (map_width, map_height) = (map_settings.width, map_settings.height);
    transform.translation.x = transform.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
    transform.translation.y = transform.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
}

////////////////////////////////////////////////////////////////////////////////
// Creature Utility Systems
////////////////////////////////////////////////////////////////////////////////

pub fn zombie_grow(time: Res<Time>, mut creatures: Query<(&mut Sprite, &CreatureSize)>) {
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

        sprite_size = sprite_size.lerp(new_size, 2. * time.delta_seconds());
        c_sprite.custom_size = Some(sprite_size);
    }
}

pub fn kill_creatures(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    bullet_query: Query<(Entity, &Transform, &FiredBy), With<Bullet>>,
    mut creatures: Query<
        (Entity, &CreatureType, &CreatureSize, &Transform),
        (Without<Bullet>, Or<(With<CreatureFollow>, With<CreatureTarget>, With<CreatureType>)>),
    >,
) {
    for (entity, c_type, c_size, c_transform) in creatures.iter_mut() {
        for (bullet_ent, bullet_transform, fired_by) in bullet_query.iter() {
            if Some(fired_by.0) != c_type.0 {
                let distance =
                    Vec2::distance(c_transform.translation.xy(), bullet_transform.translation.xy());

                if distance < (c_size.0 / 2.) {
                    commands.entity(bullet_ent).despawn_recursive();
                    damage_events.send(DamageEvent::new(entity, fired_by.0));
                }
            }
        }
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
    let zombie_vec = creatures.iter().collect::<Vec<_>>();
    let compute_task_pool = ComputeTaskPool::get();
    let creatures_per_thread = zombie_vec.len() / compute_task_pool.thread_num();
    if creatures_per_thread == 0 {
        return;
    }

    let creatures = &creatures;
    let cache_grid = &cache_grid;
    let apply_force_event_handler = Arc::new(Mutex::new(apply_force_event_handler));

    compute_task_pool.scope(|scope| {
        for chunk in zombie_vec.chunks(creatures_per_thread) {
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
                        zombie_settings::ZOMBIE_COLLISION_AVOIDANCE,
                        zombie_settings::ZOMBIE_COHESION,
                        zombie_settings::ZOMBIE_SEPERATION,
                        zombie_settings::ZOMBIE_ALIGNMENT,
                    );

                    for entity_b in cache_grid
                        .get_nearby_entities(position_a, zombie_settings::ZOMBIE_VISION)
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
                        if distance <= zombie_settings::ZOMBIE_VISION {
                            vision_count += 1;
                            average_position += position_b;
                            average_direction += direction_b.0;
                        }
                        if distance <= zombie_settings::ZOMBIE_VISION / 2.0 {
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
    let zombie_vec = creatures.iter().collect::<Vec<_>>();
    let compute_task_pool = ComputeTaskPool::get();
    let creatures_per_thread = usize::max(1, zombie_vec.len() / compute_task_pool.thread_num());
    if creatures_per_thread == 0 {
        return;
    }

    let players = &players;
    let apply_force_event_handler = Arc::new(Mutex::new(apply_force_event_handler));

    compute_task_pool.scope(|scope| {
        for chunk in zombie_vec.chunks(creatures_per_thread) {
            let apply_force_event_handler = apply_force_event_handler.clone();
            scope.spawn(async move {
                for (entity, transform, c_type, c_follow, c_target) in chunk {
                    let position_a = transform.translation.xy();

                    let (target, dist) = if let Some(t) = c_target {
                        (t.0, 1.0)
                    } else if let Some(f) = c_follow {
                        (c_type.0.unwrap(), f.0)
                    } else {
                        continue;
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
                            zombie_settings::ZOMBIE_CHASE,
                        ));
                    }
                }
            });
        }
    });
}

pub struct ZombiePlugin;
impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        // Force
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::ApplyForce)
                .run_in_state(AppState::InGame)
                .with_system(zombie_grow)
                .with_system(follow_system)
                .with_system(flocking_system)
                .into(),
        )
        .add_system(
            apply_force_event_system.run_in_state(AppState::InGame).after(SystemLabels::ApplyForce),
        );

        // movement
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::ZombieMove)
                .run_in_state(AppState::InGame)
                .with_system(creatures_follow)
                .with_system(creatures_target)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .after(SystemLabels::ZombieMove)
                .run_in_state(AppState::InGame)
                .with_system(cache_grid_update_system)
                .into(),
        );

        ////////////////////////////////
        // Death
        ////////////////////////////////
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::ZombieDamage)
                .after(SystemLabels::PlayerMove)
                .after(SystemLabels::BulletMove)
                .run_in_state(AppState::InGame)
                .with_system(kill_creatures)
                .into(),
        );
    }
}
