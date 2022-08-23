use crate::round::*;

pub mod player_settings {
    use crate::colors::*;
    use bevy::prelude::Color;

    pub const DEFAULT_ROT_SPEED: f32 = 360.;
    pub const DEFAULT_PLAYER_SIZE: f32 = 25.;
    pub const DEFAULT_MOVE_SPEED: f32 = 300.;

    pub const FOLLOW_COLLECTION_DISTANCE: f32 = 100.;
    pub const TARGET_COLLECTION_DISTANCE: f32 = 100.;

    pub const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];
}

#[derive(Debug, Default, Component)]
pub struct Player {
    pub size: f32,
    pub handle: usize,
    pub color: Color,

    /// rotation speed in radians per second
    pub rotation_speed: f32,
    /// linear speed in meters per second
    pub movement_speed: f32,
    pub attacking_zombies: u32,
    pub active_zombies: Vec<Entity>,
}

impl Player {
    pub fn new(handle: usize, color: Color) -> Self {
        Self {
            handle,
            color,
            size: player_settings::DEFAULT_PLAYER_SIZE,
            movement_speed: player_settings::DEFAULT_MOVE_SPEED,
            rotation_speed: f32::to_radians(player_settings::DEFAULT_ROT_SPEED),
            ..Default::default()
        }
    }
}

#[derive(Default, Reflect, Component, Debug)]
pub struct PlayerControls {
    pub accel: f32,
    pub steer: f32,
}

pub fn move_players(
    map_settings: Res<MapSettings>,
    mut query: Query<(&mut Transform, &PlayerControls, &Player), With<Rollback>>,
) {
    for (mut t, c, p) in query.iter_mut() {
        t.rotate_z(c.steer * p.rotation_speed * TIME_STEP);
        apply_forward_delta(&mut t, p.movement_speed, c.accel);

        // constrain cube to plane
        let (map_width, map_height) = (map_settings.width, map_settings.height);
        t.translation.x = t.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
        t.translation.y = t.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
    }
}

pub fn kill_players(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &FiredBy), With<Bullet>>,
    player_query: Query<(Entity, &Player, &Transform), (With<Player>, Without<Bullet>)>,
) {
    for (player_ent, player, player_transform) in player_query.iter() {
        for (bullet_ent, bullet_transform, fired_by) in bullet_query.iter() {
            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );

            if distance < (player.size / 2.) {
                commands.entity(bullet_ent).despawn_recursive();

                player.active_zombies.iter().for_each(|e| {
                    commands
                        .entity(*e)
                        .insert(CreatureTarget(fired_by.0))
                        .remove::<CreatureFollow>();
                });

                let attacker = player_query.get(fired_by.0).unwrap().1;
                attacker.active_zombies.iter().for_each(|e| {
                    commands
                        .entity(*e)
                        .insert(CreatureTarget(player_ent))
                        .remove::<CreatureFollow>();
                });
            }
        }
    }
}

pub fn follow_collection(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    mut players: Query<(Entity, &mut Player, &Transform)>,
    mut creature_query: Query<
        (Entity, &mut Sprite, &Transform),
        (With<CreatureType>, Without<CreatureFollow>, Without<CreatureTarget>),
    >,
) {
    for (player_ent, mut player, transform) in &mut players {
        for (creature_ent, mut sprite, _) in creature_query.iter_mut().filter(|(_, _, t)| {
            Vec2::distance(transform.translation.xy(), t.translation.xy())
                < player_settings::FOLLOW_COLLECTION_DISTANCE
        }) {
            let follow_distance = rng.range(
                creature_settings::FOLLOW_PLAYER_MIN_DISTANCE,
                creature_settings::FOLLOW_PLAYER_MAX_DISTANCE,
            );

            player.active_zombies.push(creature_ent);
            sprite.color = player.color;
            commands
                .entity(creature_ent)
                .insert(CreatureType(Some(player_ent)))
                .insert(CreatureFollow(follow_distance));
        }
    }
}

// pub fn target_collection_players(
//     mut commands: Commands,
//     rng: Res<RandomNumbers>,
//     mut players: Query<(Entity, &mut Player, &Transform)>,
//     creature_query: Query<
//         (Entity, &Transform, &CreatureType),
//         (With<CreatureFollow>, Without<CreatureTarget>),
//     >,
// ) {
//     for (player_ent, mut player, transform) in &mut players {
//         for (creature_ent, _, _) in creature_query.iter().filter(|(_, t, c)| {
//             Vec2::distance(transform.translation.xy(), t.translation.xy())
//                 < player_settings::TARGET_COLLECTION_DISTANCE
//                 && c.0.unwrap() != player_ent
//         }) {
//             let follow_distance = rng.range(
//                 creature_settings::TARGET_PLAYER_MIN_DISTANCE,
//                 creature_settings::TARGET_PLAYER_MAX_DISTANCE,
//             );

//             player.attacking_zombies += 1;
//             commands.entity(creature_ent).insert(CreatureTarget::new(player_ent, follow_distance));
//             commands
//                 .entity(player_ent)
//                 .insert(CreatureTargeted::new(creature_ent, follow_distance));
//         }
//     }
// }

// pub fn target_collection_creatures(
//     mut commands: Commands,
//     rng: Res<RandomNumbers>,
//     target_creature_query: Query<
//         (Entity, &Transform, &CreatureType),
//         (With<CreatureFollow>, Without<CreatureTarget>, Without<CreatureTargeted>),
//     >,
//     creature_query: Query<
//         (Entity, &Transform, &CreatureType),
//         (With<CreatureFollow>, Without<CreatureTarget>, Without<CreatureTargeted>),
//     >,
// ) {
//     for (target_creature_ent, &transform, target_type) in &target_creature_query {
//         for (creature_ent, _, _) in creature_query.iter().filter(|(_, t, creature_type)| {
//             Vec2::distance(transform.translation.xy(), t.translation.xy())
//                 < creature_settings::TARGET_COLLECTION_DISTANCE
//                 && target_type.0 != creature_type.0
//         }) {
//             let follow_distance = rng.range(
//                 creature_settings::TARGET_PLAYER_MIN_DISTANCE,
//                 creature_settings::TARGET_PLAYER_MAX_DISTANCE,
//             );

//             commands
//                 .entity(creature_ent)
//                 .insert(CreatureTarget::new(target_creature_ent, follow_distance));

//             commands
//                 .entity(target_creature_ent)
//                 .insert(CreatureTargeted::new(creature_ent, follow_distance));
//         }
//     }
// }
