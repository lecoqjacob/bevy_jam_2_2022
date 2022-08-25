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

    pub const CREATURE_SPEED: f32 = 210.;
    pub const CREATURE_VISION: f32 = 110.;
    pub const DEFAULT_CREATURE_SIZE: (f32, f32) = (10., 15.); // (min, max)

    pub const CREATURE_COLLISION_AVOIDANCE: f32 = 4.;
    pub const CREATURE_COHESION: f32 = 5.;
    pub const CREATURE_SEPERATION: f32 = 3.;
    pub const CREATURE_ALIGNMENT: f32 = 15.;
    pub const CREATURE_CHASE: f32 = 15.;

    pub const CREATURE_ATTACK_COOLDOWN: f32 = 1.;
}

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
        .id()
}

pub fn apply_force_event_system(
    time: Res<Time>,
    mut apply_force_event_handler: EventReader<ApplyForceEvent>,
    mut creature_query: Query<&mut crate::components::Direction>,
) {
    for ApplyForceEvent(entity, force, factor) in apply_force_event_handler.iter() {
        if let Ok(mut direction) = creature_query.get_mut(*entity) {
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
                if distance < c_follow.0 { continue } else { creature_settings::CREATURE_SPEED };

            // Move and rotate based on direction
            move_target(&time, &mut transform, direction, speed, &map_settings);
        }
    }
}

pub fn creatures_target(
    time: Res<Time>,
    mut commands: Commands,
    mut player_entity: Local<Option<Entity>>,
    mut attack_timer: Local<f32>,
    map_settings: Res<MapSettings>,
    mut player_q: Query<
        (Entity, &Transform, &Player, &mut Health),
        (With<Player>, Without<CreatureType>),
    >,
    mut query: Query<
        (Entity, &mut Transform, &crate::components::Direction, &CreatureTarget),
        (With<CreatureTarget>, Without<Player>),
    >,
) {
    if query.iter().len() > 0 {
        let can_attack = *attack_timer > creature_settings::CREATURE_ATTACK_COOLDOWN;
        *attack_timer += time.delta_seconds();

        for (creature, mut transform, direction, target) in &mut query {
            if let Some((p_entity, p_transform, player, mut health)) =
                player_q.iter_mut().find(|(p, _, _, _)| *p == target.0)
            {
                let player_translation = p_transform.translation.xy();
                let distance = player_translation.distance(transform.translation.xy());

                if distance < player.size + 10. && can_attack {
                    health.0 -= 1;
                    if health.0 <= 0 {
                        player.active_zombies.iter().for_each(|e| {
                            commands
                                .entity(*e)
                                .remove::<CreatureFollow>()
                                .remove::<CreatureTarget>();
                        });

                        commands.spawn().insert(Respawn { time: 3., color: player.color });

                        commands.entity(creature).remove::<CreatureTarget>();
                        *player_entity = Some(p_entity);
                    }
                } else {
                    move_target(
                        &time,
                        &mut transform,
                        direction,
                        creature_settings::CREATURE_SPEED,
                        &map_settings,
                    );
                };
            }
        }
    } else {
        *attack_timer = 0.0;
        if let Some(e) = *player_entity {
            commands.entity(e).despawn_recursive();
            *player_entity = None;
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

pub fn creature_grow(time: Res<Time>, mut creatures: Query<(&mut Sprite, &CreatureSize)>) {
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
    mut players: Query<(Entity, &mut Player)>,
    bullet_query: Query<(Entity, &Transform, &FiredBy), With<Bullet>>,
    mut creatures: Query<
        (Entity, &CreatureType, &CreatureSize, &Transform, &mut Health),
        (Without<Bullet>, Or<(With<CreatureFollow>, With<CreatureTarget>)>),
    >,
) {
    for (entity, c_type, c_size, c_transform, mut c_health) in creatures.iter_mut() {
        for (bullet_ent, bullet_transform, fired_by) in bullet_query.iter() {
            let distance =
                Vec2::distance(c_transform.translation.xy(), bullet_transform.translation.xy());

            if distance < (c_size.0 / 2.) {
                let attacker = players.get_mut(fired_by.0).unwrap().0;
                if c_type.0 != Some(attacker) {
                    commands.entity(bullet_ent).despawn_recursive();
                    c_health.0 -= 1;

                    if c_health.0 <= 0 {
                        commands.entity(entity).despawn_recursive();

                        if let Some(player_ent) = c_type.0 {
                            let mut player = players.get_mut(player_ent).unwrap().1;
                            if let Some(index) =
                                player.active_zombies.iter().position(|e| *e == entity)
                            {
                                player.active_zombies.remove(index);
                            }
                        }
                    }
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
                            creature_settings::CREATURE_CHASE,
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
                .with_system(creature_grow)
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
