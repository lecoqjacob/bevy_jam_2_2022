use crate::round::*;

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
                .insert(RoundEntity)
                .insert(Rollback::new(rip.next_id()));

            bullet_ready.0 = false;
        }
    }
}

pub fn move_bullet(mut query: Query<&mut Transform, With<Bullet>>) {
    for mut t in query.iter_mut() {
        apply_forward_delta(&mut t, BULLET_SPEED, 1.0);
    }
}
