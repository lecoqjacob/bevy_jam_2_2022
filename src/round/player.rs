use crate::round::*;

pub mod player_settings {
    pub const DEFAULT_ROT_SPEED: f32 = 360.;
    pub const DEFAULT_PLAYER_SIZE: f32 = 15.;
    pub const DEFAULT_MOVE_SPEED: f32 = 300.;
}

#[derive(Debug, Default, Component)]
pub struct Player {
    pub size: f32,
    pub handle: usize,

    /// rotation speed in radians per second
    pub rotation_speed: f32,
    /// linear speed in meters per second
    pub movement_speed: f32,
}

impl Player {
    pub fn new(handle: usize) -> Self {
        Self {
            handle,
            size: player_settings::DEFAULT_PLAYER_SIZE,
            movement_speed: player_settings::DEFAULT_MOVE_SPEED,
            rotation_speed: f32::to_radians(player_settings::DEFAULT_ROT_SPEED),
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
    player_query: Query<(Entity, &Player, &Transform), (With<Player>, Without<Bullet>)>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (player_ent, player, player_transform) in player_query.iter() {
        for (bullet_ent, bullet_transform) in bullet_query.iter() {
            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );
            if distance < (player.size / 2.) {
                commands.entity(player_ent).despawn_recursive();
                commands.entity(bullet_ent).despawn_recursive();
            }
        }
    }
}
