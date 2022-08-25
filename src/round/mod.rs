// This is the folder to handle the `round` or `in_game` state

use crate::prelude::*;
use bevy::{sprite::MaterialMesh2dBundle, window::WindowResized};
use bytemuck::{Pod, Zeroable};

mod bullet;
mod input;
mod player;
mod ui;
mod zombie;

pub use bullet::*;
pub use input::*;
pub use player::*;
use rand::{seq::SliceRandom, thread_rng};
pub use ui::*;
pub use zombie::*;

pub const ZOMBIE_RESPAWN_RATE: f32 = 15.0; // seconds
pub const TOTAL_ZOMBIES: usize = 100;
pub const COLLECTED_ZOMBIES_TO_WIN: usize = 25;

#[derive(Component)]
pub struct RoundEntity;

pub fn setup_round(
    windows: Res<Windows>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut resize_events: EventWriter<WindowResized>,
) {
    commands.init_resource::<CacheGrid>();

    // Despawn All Players/Bullets if they exist
    for player in player_query.iter() {
        commands.entity(player).despawn_recursive();
    }
    for bullet in bullet_query.iter() {
        commands.entity(bullet).despawn_recursive();
    }

    // Create Viewports
    let main_wnd = windows.get_primary().unwrap();
    resize_events.send(WindowResized {
        id: main_wnd.id(),
        width: main_wnd.width(),
        height: main_wnd.height(),
    });

    let mut colors = player_settings::PLAYER_COLORS;
    colors.shuffle(&mut thread_rng());
    for color in colors.iter().take(2) {
        spawn_events.send(SpawnEvent { spawn_type: SpawnType::Player, color: Some(*color) });
    }

    for _ in 0..50 {
        spawn_events.send(SpawnEvent { spawn_type: SpawnType::Zombie, ..default() });
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Spawning
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawning(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    meshes: Res<MeshAssets>,
    settings: Res<MapSettings>,
    materials: Res<MaterialAssets>,
    mut evs: EventReader<SpawnEvent>,
) {
    for SpawnEvent { color, spawn_type } in evs.iter() {
        match spawn_type {
            SpawnType::Zombie => {
                let direction_vector =
                    Vec2::new(rng.rand::<f32>() * 2.0 - 1.0, rng.rand::<f32>() * 2.0 - 1.0)
                        .normalize();

                let (x, y) = random_map_point(settings.width, settings.height, &rng);
                let transform =
                    Transform::default().with_translation(Vec3::new(x, y, 10.0)).with_rotation(
                        Quat::from_rotation_z(-direction_vector.x.atan2(direction_vector.y)),
                    );

                let size = zombie_settings::DEFAULT_ZOMBIE_SIZE.0;
                spawn_zombie(&mut commands, transform, direction_vector, size);
            }
            SpawnType::Player => {
                let (x, y) = random_map_point(settings.width, settings.height, &rng);
                let transform = Transform::default().with_translation(Vec3::new(x, y, 10.0));
                let color = color.unwrap();

                println!("Spawning player with color: {:?}", color);
                println!("Spawning player with color: {:?}", get_color_name(color));
                println!("MATERIAL: : {:?}", materials.get(color));
                spawn_player(
                    &mut commands,
                    transform,
                    color,
                    meshes.ring.clone(),
                    materials.get(color),
                );
            }
        }
    }
}

pub fn random_spawn_creatures(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut spawn_event: EventWriter<SpawnEvent>,
    total_zombie_follow: Query<&CreatureFollow>,
) {
    if total_zombie_follow.iter().count() < TOTAL_ZOMBIES {
        *timer += time.delta_seconds();
        if *timer >= ZOMBIE_RESPAWN_RATE {
            *timer = 0.0;
            spawn_event.send(SpawnEvent { color: None, spawn_type: SpawnType::Zombie });
        }
    }
}

pub fn random_map_point(width: f32, height: f32, rng: &RandomNumbers) -> (f32, f32) {
    let map_width = width / 2.;
    let map_height = height / 2.;
    let x = rng.range(-map_width, map_width);
    let y = rng.range(-map_height, map_height);

    (x, y)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Game Utility
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn handle_damage_events(
    mut commands: Commands,
    mut q: Query<&mut Health>,
    mut damages: EventReader<DamageEvent>,
    mut players: Query<&mut Player, Without<CreatureType>>,
    mut zombies: Query<&mut CreatureType, Without<Player>>,
) {
    for DamageEvent { victim, attacker } in damages.iter() {
        if let Ok(mut health) = q.get_mut(*victim) {
            health.0 -= 1;

            // Handle Player Cases
            if let Ok(player) = players.get_mut(*victim) {
                if health.0 <= 0 {
                    player.active_zombies.iter().for_each(|e| {
                        println!("Despawning zombie: {:?}", e);
                        commands.entity(*e).remove::<CreatureFollow>().remove::<CreatureTarget>();
                    });

                    commands.entity(*victim).despawn_descendants();

                    commands
                        .entity(*victim)
                        .remove_bundle::<PlayerBundle>()
                        .insert(Dead)
                        .insert(Clock::new(3.));
                } else {
                    player.active_zombies.iter().for_each(|e| {
                        commands.entity(*e).insert(CreatureTarget(*attacker));
                    });
                }
            }

            // Handle Zombie Cases
            if let Ok(z_type) = zombies.get_mut(*victim) {
                if health.0 <= 0 {
                    commands.entity(*victim).despawn_recursive();

                    if let Some(parent) = z_type.0 {
                        let mut player = players.get_mut(parent).unwrap();
                        player.active_zombies.retain(|e| *e != *victim);
                    }
                } else {
                }
            }
        }
    }
}

pub fn check_win(mut commands: Commands, player: Query<&Player, Changed<Player>>) {
    for p in player.iter() {
        if p.active_zombies.len() >= COLLECTED_ZOMBIES_TO_WIN {
            commands.insert_resource(NextState(AppState::Win));
            commands.insert_resource(MatchData {
                result: format!("Player {:?} won!", get_color_name(p.color)),
            });
        }
    }
}

pub fn cleanup_round(mut commands: Commands) {
    commands.remove_resource::<CacheGrid>();
}

pub struct RoundPlugin;
impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyForceEvent>();
        app.add_event::<SpawnEvent>();
        app.add_event::<DamageEvent>();

        // Game Plugins
        app.add_plugin(PlayerPlugin)
            .add_plugin(ZombiePlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(RoundUIPlugin);

        app.add_enter_system(AppState::InGame, setup_round);
        app.add_system(handle_damage_events.run_on_event::<DamageEvent>());

        ////////////////////////////////
        // Spawning
        ////////////////////////////////
        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::Spawning)
                .run_in_state(AppState::InGame)
                .with_system(spawning)
                .with_system(random_spawn_creatures)
                .into(),
        );

        ////////////////////////////////
        // Cleanup
        ////////////////////////////////
        app.add_system_set(
            ConditionSet::new().run_in_state(AppState::InGame).with_system(check_win).into(),
        )
        .add_exit_system_set(
            AppState::InGame,
            ConditionSet::new()
                .with_system(cleanup_round)
                .with_system(despawn_all_with::<RoundEntity>)
                .into(),
        );
    }
}
