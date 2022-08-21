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

pub const FPS: usize = 60;
pub const TIME_STEP: f32 = 1.0 / FPS as f32;

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GGRSConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(round::input)
            .register_rollback_type::<Transform>()
            .register_rollback_type::<BulletReady>()
            .register_rollback_type::<FrameCount>()
            .register_rollback_type::<Checksum>()
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(
                        ROLLBACK_SYSTEMS,
                        SystemStage::parallel()
                            .with_system(apply_inputs.label(SystemLabels::Input))
                            .with_system(move_players.after(SystemLabels::Input))
                            .with_system(increase_frame_count)
                            .with_system(reload_bullet)
                            .with_system(fire_bullets.after(move_players).after(reload_bullet)) // .with_system(move_bullet),
                            .with_system(move_bullet),
                    )
                    .with_stage_after(
                        ROLLBACK_SYSTEMS,
                        CHECKSUM_UPDATE,
                        SystemStage::parallel().with_system(checksum_players),
                    ),
            )
            .build(app);
    }
}
