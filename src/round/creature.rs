use crate::round::*;
use bevy::{math::Vec3Swizzles, tasks::ComputeTaskPool};
use parking_lot::Mutex;
use std::sync::Arc;

pub const CREATURE_SPEED: f32 = 100.;

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

pub fn flocking_system(
    cache_grid: Res<CacheGrid>,
    apply_force_event_handler: EventWriter<ApplyForceEvent>,
    creatures: Query<(Entity, &crate::components::Direction, &Transform, &CreatureType)>,
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

                    let vision = 15.0;
                    let size = Vec2::new(5.0, 10.0);
                    let collision_avoidance = 4.0;

                    let cohesion = 5.0;
                    let separation = 3.0;
                    let alignment = 15.0;

                    for entity_b in cache_grid.get_nearby_entities(position_a, vision) {
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
                        if distance <= vision {
                            vision_count += 1;
                            average_position += position_b;
                            average_direction += direction_b.0;
                        }
                        if distance <= vision / 2.0 {
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

pub fn chase_system(
    cache_grid: Res<CacheGrid>,
    players: Query<&Transform, With<Player>>,
    creatures: Query<(Entity, &Transform, &CreatureTarget), Without<Player>>,
    apply_force_event_handler: EventWriter<ApplyForceEvent>,
) {
    let creature_vec = creatures.iter().collect::<Vec<_>>();
    let compute_task_pool = ComputeTaskPool::get();
    let creatures_per_thread = creature_vec.len() / compute_task_pool.thread_num();
    if creatures_per_thread == 0 {
        return;
    }

    let cache_grid = &cache_grid;
    let players = &players;
    let apply_force_event_handler = Arc::new(Mutex::new(apply_force_event_handler));

    compute_task_pool.scope(|scope| {
        for chunk in creature_vec.chunks(creatures_per_thread) {
            let apply_force_event_handler = apply_force_event_handler.clone();
            scope.spawn(async move {
                for (entity_a, transform_a, target) in chunk {
                    let id_a = *entity_a;
                    let position_a = transform_a.translation.xy();
                    let mut closest_target = (0.0, None);

                    const VISION: f32 = 50.0;
                    const CHASE: f32 = 15.0;

                    if let Some(entity_b) = cache_grid
                        .get_nearby_entities(position_a, VISION)
                        .iter()
                        .find(|e| **e == target.1)
                    {
                        let position_b = match players.get(*entity_b) {
                            Ok(player_transform) => player_transform.translation.xy(),
                            Err(_) => continue,
                        };

                        let distance = position_a.distance(position_b);
                        if distance <= VISION {
                            closest_target = match closest_target {
                                (_, None) => (distance, Some(position_b)),
                                (old_distance, Some(_)) => {
                                    if old_distance > distance {
                                        (distance, Some(position_b))
                                    } else {
                                        closest_target
                                    }
                                }
                            };
                        }
                    }

                    let closest_position = match closest_target {
                        (_, Some(position)) => position,
                        (_, None) => continue,
                    };
                    let chase_direction = (closest_position - position_a).normalize();
                    apply_force_event_handler.lock().send(ApplyForceEvent(
                        id_a,
                        chase_direction,
                        CHASE,
                    ));
                }
            });
        }
    });
}

pub fn move_creatures(
    player_q: Query<(&Transform, &Player), Without<CreatureType>>,
    mut query: Query<
        (&mut Transform, &crate::components::Direction, &CreatureTarget),
        Without<Player>,
    >,
) {
    for (mut transform, direction, target) in query.iter_mut() {
        let p_target = player_q.iter().find(|(_, player)| player.handle == target.0).unwrap();
        let player_translation = p_target.0.translation.xy();
        let to_player = (player_translation - transform.translation.xy()).normalize();

        let distance = Vec2::distance(p_target.0.translation.xy(), transform.translation.xy());
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

        let (acc, speed) = if distance > 25.0 { (1.0, CREATURE_SPEED) } else { (0.0, 0.0) };

        let x = direction.0.x * acc * speed * TIME_STEP;
        let y = direction.0.y * acc * speed * TIME_STEP;

        if x.is_nan() || y.is_nan() {
            continue;
        }

        transform.translation.x += x;
        transform.translation.y += y;
        transform.rotation = rotate_to_player;
    }
}
