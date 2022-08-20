use crate::round::*;

pub struct OnlineRoundPlugin;
impl Plugin for OnlineRoundPlugin {
    fn build(&self, app: &mut App) {
        // online round
        app.add_enter_system_set(
            AppState::RoundOnline,
            ConditionSet::new().with_system(setup_round).with_system(spawn_players).into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundOnline)
                .with_system(print_p2p_events)
                .with_system(check_win)
                .into(),
        )
        .add_exit_system(AppState::RoundOnline, cleanup);
    }
}
