use crate::prelude::*;
use crate::round;

mod checksum;
pub use checksum::*;

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type State = u8;
    type Input = round::GameInput;
    type Address = String;
}

// Time
pub const FPS: usize = 60;
pub const TIME_STEP: f32 = 1.0 / FPS as f32;

pub const NUM_PLAYERS: usize = 2;

pub const MAX_PREDICTION: usize = 12;
pub const INPUT_DELAY: usize = 2;
pub const CHECK_DISTANCE: usize = 2;

// Stages
#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
enum RollbackStages {
    Rollback,
    Creature,
    Checksum,
}
pub const ROLLBACK_SYSTEMS: &str = "rollback_systems";
pub const CHECKSUM_UPDATE: &str = "checksum_update";

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GGRSConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(round::input)
            .register_rollback_type::<FrameCount>()
            .register_rollback_type::<Checksum>()
            .register_rollback_type::<Transform>()
            .register_rollback_type::<BulletReady>()
            .register_rollback_type::<crate::components::Direction>()
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(
                        RollbackStages::Rollback,
                        SystemStage::parallel()
                            .with_system(apply_inputs.label(SystemLabels::Input))
                            .with_system(move_players.after(SystemLabels::Input))
                            .with_system(increase_frame_count)
                            .with_system(reload_bullet)
                            .with_system(fire_bullets.after(move_players).after(reload_bullet)) // .with_system(move_bullet),
                            .with_system(move_bullet)
                            .with_system(kill_players.after(move_bullet).after(move_players)),
                    )
                    .with_stage_after(
                        RollbackStages::Rollback,
                        RollbackStages::Creature,
                        SystemStage::parallel()
                            .with_system(follow_collection)
                            // .with_system(target_collection_players)
                            // .with_system(target_collection_creatures)
                            .with_system(creatures_follow)
                            // .with_system(creatures_target)
                            .with_system(cache_grid_update_system.after(creatures_follow))
                            .with_system_set(
                                SystemSet::new()
                                    .label("force_adding")
                                    .with_system(follow_system)
                                    .with_system(flocking_system.after(follow_system)),
                            )
                            .with_system(apply_force_event_system.after("force_adding")),
                    )
                    .with_stage_after(
                        RollbackStages::Creature,
                        RollbackStages::Checksum,
                        SystemStage::parallel().with_system(checksum_players),
                    ),
            )
            .build(app);
    }
}
