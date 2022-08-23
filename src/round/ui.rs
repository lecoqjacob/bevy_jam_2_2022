use crate::round::*;

#[derive(Component)]
pub struct ZombieText;

pub fn setup_round_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Captured Zombies: ",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::GOLD,
                        font: fonts.fira_sans.clone(),
                    },
                ),
            ])
            .with_style(Style { align_self: AlignSelf::FlexEnd, ..default() }),
        )
        .insert(ZombieText)
        .insert(RoundEntity);
}

fn update_round_text(
    local: Res<LocalHandles>,
    players: Query<(Entity, &Player)>,
    mut query: Query<&mut Text, With<ZombieText>>,
) {
    let local_handle = local.handles[0];
    let player = players.iter().find(|(_, p)| p.handle == local_handle);

    for mut text in &mut query {
        if let Some(player) = player {
            text.sections[1].value = player.1.active_zombies.len().to_string();
        }
    }
}

pub struct RoundUIPlugin;
impl Plugin for RoundUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::RoundLocal, setup_round_ui);
        app.add_enter_system(AppState::RoundOnline, setup_round_ui);

        app.add_system(update_round_text.run_in_state(AppState::RoundLocal));
        app.add_system(update_round_text.run_in_state(AppState::RoundOnline));
    }
}
