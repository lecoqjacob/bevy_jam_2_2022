use crate::round::*;

pub struct LocalRoundPlugin;
impl Plugin for LocalRoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::RoundLocal,
            ConditionSet::new().with_system(setup_round).with_system(spawn_players).into(),
        )
        .add_system(check_win.run_in_state(AppState::RoundLocal))
        .add_exit_system(AppState::RoundLocal, cleanup);
    }
}
