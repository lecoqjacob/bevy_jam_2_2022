use bevy::math::Vec3Swizzles;

use crate::round::*;

pub mod player_settings {
    use crate::colors::*;
    use bevy::prelude::Color;

    pub const DEFAULT_ROT_SPEED: f32 = 360.;
    pub const DEFAULT_PLAYER_SIZE: f32 = 25.;
    pub const DEFAULT_MOVE_SPEED: f32 = 200.;

    pub const FOLLOW_COLLECTION_DISTANCE: f32 = 100.;
    pub const TARGET_COLLECTION_DISTANCE: f32 = 100.;

    pub const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];
}

#[derive(Reflect, Debug, Default, Component, Clone)]
pub struct Player {
    pub size: f32,
    pub color: Color,

    /// rotation speed in radians per second
    pub rotation_speed: f32,
    /// linear speed in meters per second
    pub movement_speed: f32,
    pub attacking_zombies: u32,
    pub active_zombies: Vec<Entity>,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            size: player_settings::DEFAULT_PLAYER_SIZE,
            movement_speed: player_settings::DEFAULT_MOVE_SPEED,
            rotation_speed: f32::to_radians(player_settings::DEFAULT_ROT_SPEED),
            ..Default::default()
        }
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    transform: Transform,
    color: Color,
    ring_mesh: Handle<Mesh>,
    color_mat: Handle<ColorMaterial>,
) {
    println!("spawn_player: transform={:?}", transform);

    let player_comp = Player::new(color);
    let player = commands
        .spawn_bundle(SpriteBundle {
            transform,
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(player_comp.size, player_comp.size)),
                ..default()
            },
            ..default()
        })
        .insert(player_comp)
        .insert(PlayerControls::default())
        .insert(BulletReady(true))
        .insert(Health(10))
        .insert(RoundEntity)
        .id();

    commands.entity(player).add_children(|p| {
        p.spawn_bundle(SpriteBundle {
            transform: transform.with_translation(Vec3::new(0., 10., 5.)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(5., 15.)),
                ..default()
            },
            ..default()
        })
        .insert(RoundEntity);

        p.spawn_bundle(MaterialMesh2dBundle {
            transform: transform.with_translation(Vec3::new(0., 0., 0.)),
            mesh: ring_mesh.into(),
            material: color_mat,
            ..default()
        })
        .insert(RoundEntity);
    });
}

#[derive(Default, Reflect, Component, Debug)]
pub struct PlayerControls {
    pub accel: f32,
    pub steer: f32,
    pub firing: bool,
}

pub fn move_players(
    time: Res<Time>,
    map_settings: Res<MapSettings>,
    mut query: Query<(&mut Transform, &PlayerControls, &Player)>,
) {
    for (mut t, c, p) in query.iter_mut() {
        t.rotate_z(c.steer * p.rotation_speed * time.delta_seconds());
        apply_forward_delta(&time, &mut t, p.movement_speed, c.accel);

        // constrain cube to plane
        let (map_width, map_height) = (map_settings.width, map_settings.height);
        t.translation.x = t.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
        t.translation.y = t.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
    }
}

pub fn kill_players(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &FiredBy), With<Bullet>>,
    mut player_query: Query<
        (Entity, &Player, &Transform, &mut Health),
        (With<Player>, Without<Bullet>),
    >,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (player_ent, target_player, player_transform, mut p_health) in player_query.iter_mut() {
        for (bullet_ent, bullet_transform, fired_by) in bullet_query.iter() {
            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );

            if distance < (target_player.size / 2.) {
                commands.entity(bullet_ent).despawn_recursive();

                damage_events.send(DamageEvent(player_ent));

                // p_health.0 -= 1;
                // if p_health.0 <= 0 {
                //     target_player.active_zombies.iter().for_each(|e| {
                //         commands.entity(*e).remove::<CreatureFollow>().remove::<CreatureTarget>();
                //     });

                //     commands.spawn().insert(Respawn { time: 3., color: target_player.color });
                //     commands.entity(player_ent).despawn_recursive();
                // } else {
                //     target_player.active_zombies.iter().for_each(|e| {
                //         commands
                //             .entity(*e)
                //             // .remove::<CreatureFollow>()
                //             .insert(CreatureTarget(fired_by.0));
                //     });
                // }
            }
        }
    }
}

pub fn respawn_players(
    time: Res<Time>,
    mut commands: Commands,
    mut respawns: Query<(Entity, &mut Respawn)>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    for (ent, mut respawn) in &mut respawns {
        respawn.time -= time.delta_seconds();

        if respawn.time <= 0.0 {
            commands.entity(ent).despawn_recursive();
            spawn_events
                .send(SpawnEvent { spawn_type: SpawnType::Player, color: Some(respawn.color) });
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Creature Interaction
///////////////////////////////////////////////////////////////////////////////

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

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        ////////////////////////////////
        // Input
        ////////////////////////////////
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::Input)
                .run_in_state(AppState::InGame)
                .with_system(input.chain(apply_inputs))
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .label(SystemLabels::PlayerMove)
                .after(SystemLabels::Input)
                .run_in_state(AppState::InGame)
                .with_system(move_players)
                .into(),
        );

        ////////////////////////////////
        // Collection
        ////////////////////////////////
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::Collection)
                .run_in_state(AppState::InGame)
                .with_system(follow_collection)
                .into(),
        );

        ////////////////////////////////
        // Death
        ////////////////////////////////
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::PlayerDamage)
                .after(SystemLabels::PlayerMove)
                .after(SystemLabels::BulletMove)
                .run_in_state(AppState::InGame)
                .with_system(kill_players)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .after(SystemLabels::PlayerDamage)
                .run_in_state(AppState::InGame)
                .with_system(respawn_players)
                .into(),
        );
    }
}
