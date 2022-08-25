use crate::round::*;

pub fn reload_bullet(mut query: Query<(&PlayerControls, &mut BulletReady)>) {
    for (controls, mut bullet_ready) in query.iter_mut() {
        if !controls.firing {
            bullet_ready.0 = true;
        }
    }
}

pub fn fire_bullets(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut player_query: Query<(Entity, &Transform, &Player, &PlayerControls, &mut BulletReady)>,
) {
    for (player_ent, transform, player, controls, mut bullet_ready) in player_query.iter_mut() {
        if controls.firing && bullet_ready.0 {
            let movement_direction = transform.rotation * Vec3::Y;
            let translation = transform.translation + movement_direction * player.size;

            commands
                .spawn_bundle(SpriteBundle {
                    transform: transform.with_translation(translation),
                    texture: textures.bullet.clone(),
                    sprite: Sprite { custom_size: Some(Vec2::new(5., 5.)), ..default() },
                    ..default()
                })
                .insert(Bullet)
                .insert(FiredBy(player_ent))
                .insert(RoundEntity);

            bullet_ready.0 = false;
        }
    }
}

pub fn move_bullet(time: Res<Time>, mut query: Query<&mut Transform, With<Bullet>>) {
    for mut t in query.iter_mut() {
        apply_forward_delta(&time, &mut t, BULLET_SPEED, 1.0);
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::BulletReload)
                .run_in_state(AppState::InGame)
                .with_system(reload_bullet)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::BulletMove)
                .after(SystemLabels::PlayerMove)
                .after(SystemLabels::BulletReload)
                .run_in_state(AppState::InGame)
                .with_system(fire_bullets)
                .with_system(move_bullet)
                .into(),
        );
    }
}
