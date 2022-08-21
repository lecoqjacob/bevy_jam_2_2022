use crate::round::*;

pub mod player_settings {
    pub const DEFAULT_ROT_SPEED: f32 = 360.;
    pub const DEFAULT_PLAYER_SIZE: f32 = 15.;
    pub const DEFAULT_MOVE_SPEED: f32 = 300.;
}

#[derive(Default, Component)]
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

pub fn reload_bullet(
    inputs: Res<Vec<(GameInput, InputStatus)>>,
    mut query: Query<(&Player, &mut BulletReady)>,
) {
    for (player, mut bullet_ready) in query.iter_mut() {
        let input = inputs[player.handle].0.inp;
        if !is_firing(input) {
            bullet_ready.0 = true;
        }
    }
}

pub fn fire_bullets(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut rip: ResMut<RollbackIdProvider>,
    inputs: Res<Vec<(GameInput, InputStatus)>>,
    mut player_query: Query<(&Transform, &Player, &mut BulletReady)>,
) {
    for (transform, player, mut bullet_ready) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if is_firing(input.inp) && bullet_ready.0 {
            let movement_direction = transform.rotation * Vec3::Y;
            let trans = transform.translation + movement_direction * player.size;

                commands
                .spawn_bundle(SpriteBundle {
                    transform: transform.with_translation(trans),
                    texture: textures.bullet.clone(),
                    sprite: Sprite { custom_size: Some(Vec2::new(5., 5.)), ..default() },
                    ..default()
                })
                .insert(Bullet)
                .insert(Rollback::new(rip.next_id()));

            bullet_ready.0 = false;
        }
    }
}
