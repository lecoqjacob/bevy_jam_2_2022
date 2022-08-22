use crate::round::*;
use bevy::{math::Vec3Swizzles, tasks::ComputeTaskPool};
use parking_lot::Mutex;
use std::sync::Arc;

pub mod creature_settings {
    pub const FOLLOW_PLAYER_MIN_DISTANCE: f32 = 25.;
    pub const FOLLOW_PLAYER_MAX_DISTANCE: f32 = 50.;

    pub const CREATURE_SPEED: f32 = 100.;
    pub const CREATURE_VISION: f32 = 110.;

    pub const CREATURE_COLLISION_AVOIDANCE: f32 = 4.;
    pub const CREATURE_COHESION: f32 = 5.;
    pub const CREATURE_SEPERATION: f32 = 3.;
    pub const CREATURE_ALIGNMENT: f32 = 15.;
    pub const CREATURE_CHASE: f32 = 15.;
}

pub struct ApplyForceEvent(pub Entity, pub Vec2, pub f32);

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

pub fn creatures_follow(
    map_settings: Res<MapSettings>,
    player_q: Query<(Entity, &Transform), (With<Player>, Without<Creature>)>,
    mut query: Query<
        (&mut Transform, &crate::components::Direction, &CreatureFollow),
        Without<Player>,
    >,
) {
    for (mut transform, direction, follow) in query.iter_mut() {
        let player_transform =
            player_q.iter().find(|(p, _)| *p == follow.target).map(|(_, t)| t).unwrap();
        let player_translation = player_transform.translation.xy();

        let distance = Vec2::distance(player_translation, transform.translation.xy());
        let (acc, speed) = if distance > creature_settings::FOLLOW_PLAYER_MAX_DISTANCE {
            (1.0, creature_settings::CREATURE_SPEED)
        } else {
            (0.0, 0.0)
        };

        let x = direction.0.x * acc * speed * TIME_STEP;
        let y = direction.0.y * acc * speed * TIME_STEP;

        if x.is_nan() || y.is_nan() {
            continue;
        }

        // Apply movement
        transform.translation.x += x;
        transform.translation.y += y;

        // Apply Rotation
        let to_player = (player_translation - transform.translation.xy()).normalize();
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
        transform.rotation = rotate_to_player;

        let (map_width, map_height) = (map_settings.width, map_settings.height);
        // let pos_bounds = (map_height / 2.0 - 10., map_height / 2.0 - 10.);
        // let neg_bounds = (-map_height / 2.0 - 10., -map_height / 2.0 - 10.);

        // Clamp to map bounds
        transform.translation.x = transform.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
        transform.translation.y =
            transform.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
    }
}

pub fn flocking_system(
    cache_grid: Res<CacheGrid>,
    apply_force_event_handler: EventWriter<ApplyForceEvent>,
    creatures: Query<(Entity, &crate::components::Direction, &Transform, &Creature)>,
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
                for (entity_a, _, transform_a, type_a) in chunk {
                    let entity_a = *entity_a;
                    let type_a = *type_a;
                    let position_a = transform_a.translation.xy();

                    let mut average_position = Vec2::ZERO; // Cohesion
                    let mut average_direction = Vec2::ZERO; // Alignment
                    let mut average_close_position = Vec2::ZERO; // Separation

                    let mut vision_count = 0;
                    let mut half_vision_count = 0;

                    let size = Vec2::new(5.0, 10.0);

                    let (collision_avoidance, cohesion, separation, alignment) = (
                        creature_settings::CREATURE_COLLISION_AVOIDANCE,
                        creature_settings::CREATURE_COHESION,
                        creature_settings::CREATURE_SEPERATION,
                        creature_settings::CREATURE_ALIGNMENT,
                    );

                    for entity_b in cache_grid
                        .get_nearby_entities(position_a, creature_settings::CREATURE_VISION)
                    {
                        if entity_a == entity_b {
                            continue;
                        }
                        let get_creature = creatures.get(entity_b);
                        if get_creature.is_err() {
                            continue;
                        }
                        let (_, direction_b, transform_b, type_b) = get_creature.unwrap();
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
    creatures: Query<(Entity, &Transform, &CreatureFollow), Without<Player>>,
    apply_force_event_handler: EventWriter<ApplyForceEvent>,
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
                for (entity_a, transform_a, follow) in chunk {
                    let id_a = *entity_a;
                    let position_a = transform_a.translation.xy();
                    let mut closest_target = (0.0, None);

                    let target = follow.target;
                    let player_position = match players.get(target) {
                        Ok(player_transform) => player_transform.translation.xy(),
                        Err(_) => continue,
                    };

                    let distance = position_a.distance(player_position);
                    if distance <= creature_settings::CREATURE_VISION {
                        closest_target = match closest_target {
                            (_, None) => (distance, Some(player_position)),
                            (old_distance, Some(_)) => {
                                if old_distance > distance {
                                    (distance, Some(player_position))
                                } else {
                                    closest_target
                                }
                            }
                        };
                    }

                    let closest_position = match closest_target {
                        (_, Some(position)) => position,
                        (_, None) => continue,
                    };
                    let chase_direction = (closest_position - position_a).normalize();
                    apply_force_event_handler.lock().send(ApplyForceEvent(
                        id_a,
                        chase_direction,
                        creature_settings::CREATURE_CHASE,
                    ));
                }
            });
        }
    });
}
