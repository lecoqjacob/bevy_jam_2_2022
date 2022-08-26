use bevy::math::Vec3Swizzles;

use crate::round::*;

use self::player_settings::SPEED_MULTIPLIER;

pub mod player_settings {
    use crate::colors::*;
    use bevy::prelude::Color;

    pub const SPEED_MULTIPLIER: f32 = 1.2;
    pub const DEFAULT_ROT_SPEED: f32 = 250.;
    pub const DEFAULT_PLAYER_SIZE: f32 = 25.;
    pub const DEFAULT_MOVE_SPEED: f32 = 200.;

    pub const FOLLOW_COLLECTION_DISTANCE: f32 = 100.;
    pub const TARGET_COLLECTION_DISTANCE: f32 = 100.;

    pub const PLAYER_COLORS: [Color; 4] = [BLUE, RED, PURPLE, GREEN];
}

///////////////////////////////////////////////////////////////////////////////
// Player Components
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Component, Clone)]
pub struct Player {
    pub size: f32,
    pub color: Color,
    pub handle: usize,

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
            color,
            handle,
            size: player_settings::DEFAULT_PLAYER_SIZE,
            movement_speed: player_settings::DEFAULT_MOVE_SPEED,
            rotation_speed: f32::to_radians(player_settings::DEFAULT_ROT_SPEED),
            ..Default::default()
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite: SpriteBundle,
    health: Health,
    ready: BulletReady,
    controls: PlayerControls,
    round_entity: RoundEntity,
}

impl PlayerBundle {
    pub fn new(transform: Transform, color: Color, texture: Handle<Image>) -> Self {
        Self {
            health: Health(10),
            sprite: SpriteBundle {
                transform,
                texture,
                sprite: Sprite {
                    color,
                    flip_y: true,
                    custom_size: Some(Vec2::new(
                        player_settings::DEFAULT_PLAYER_SIZE,
                        player_settings::DEFAULT_PLAYER_SIZE,
                    )),
                    ..default()
                },
                ..default()
            },
            ready: BulletReady(true),
            round_entity: RoundEntity,
            controls: PlayerControls::default(),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct BulletReady(pub bool);

#[derive(Component, Default, Debug)]
pub struct HealthBar;

///////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(
    commands: &mut Commands,
    transform: Transform,
    handle: usize,
    color: Color,
    texture: Handle<Image>,
    ring_mesh: Handle<Mesh>,
    color_mat: Handle<ColorMaterial>,
) {
    let player = commands
        .spawn_bundle(PlayerBundle::new(transform, color, texture))
        .insert(Player::new(handle, color))
        .id();

    commands.entity(player).add_children(|p| {
        p.spawn_bundle(SpriteBundle {
            transform: transform.with_translation(Vec3::new(0., -25., 10.)),
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(15., 5.)),
                ..default()
            },
            ..default()
        })
        .insert(HealthBar)
        .insert(RoundEntity);

        p.spawn_bundle(MaterialMesh2dBundle {
            material: color_mat,
            mesh: ring_mesh.into(),
            transform: transform.with_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(RoundEntity);
    });
}

#[derive(Default, Component, Debug)]
pub struct PlayerControls {
    pub accel: f32,
    pub steer: f32,
    pub firing: bool,
    pub shift: bool,
}

pub fn move_players(
    time: Res<Time>,
    map_settings: Res<MapSettings>,
    mut query: Query<(&mut Transform, &PlayerControls, &Player)>,
) {
    for (mut t, c, p) in query.iter_mut() {
        t.rotate_z(c.steer * p.rotation_speed * time.delta_seconds());
        apply_forward_delta(
            &time,
            &mut t,
            p.movement_speed,
            if c.shift { c.accel * SPEED_MULTIPLIER } else { c.accel },
        );

        // constrain cube to plane
        let (map_width, map_height) = (map_settings.width, map_settings.height);
        t.translation.x = t.translation.x.clamp(-map_width / 2.0, map_width / 2.0);
        t.translation.y = t.translation.y.clamp(-map_height / 2.0, map_height / 2.0);
    }
}

pub fn kill_players(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    bullet_query: Query<(Entity, &Transform, &FiredBy), With<Bullet>>,
    player_query: Query<(Entity, &Player, &Transform), (With<Player>, Without<Bullet>)>,
) {
    for (player_ent, target_player, player_transform) in player_query.iter() {
        for (bullet_ent, bullet_transform, fired_by) in bullet_query.iter() {
            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );

            if distance < (target_player.size / 2.) {
                commands.entity(bullet_ent).despawn_recursive();
                damage_events.send(DamageEvent::new(player_ent, fired_by.0));
            }
        }
    }
}

pub fn respawn_players(
    time: Res<Time>,
    mut commands: Commands,
    mut respawns: Query<(Entity, &Player, &mut Clock), With<Dead>>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    for (ent, player, mut clock) in &mut respawns {
        clock.current -= time.delta_seconds();

        if clock.current <= 0.0 {
            commands.entity(ent).despawn_recursive();
            spawn_events.send(SpawnEvent {
                color: Some(player.color),
                handle: Some(player.handle),
                spawn_type: SpawnType::Player,
            });
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
    mut zombie_query: Query<
        (Entity, &mut Sprite, &Transform),
        (With<CreatureType>, Without<CreatureFollow>, Without<CreatureTarget>),
    >,
) {
    for (player_ent, mut player, transform) in &mut players {
        for (zombie_ent, mut sprite, _) in zombie_query.iter_mut().filter(|(_, _, t)| {
            Vec2::distance(transform.translation.xy(), t.translation.xy())
                < player_settings::FOLLOW_COLLECTION_DISTANCE
        }) {
            let follow_distance = rng.range(
                zombie_settings::FOLLOW_PLAYER_MIN_DISTANCE,
                zombie_settings::FOLLOW_PLAYER_MAX_DISTANCE,
            );

            player.active_zombies.push(zombie_ent);
            sprite.color = player.color;
            commands
                .entity(zombie_ent)
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
                .before(SystemLabels::CameraMove)
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
