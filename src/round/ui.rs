use crate::round::*;

#[derive(Component)]
pub struct ZombieText;

pub fn setup_round_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    info!("setup_round_ui");

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

    // commands
    //     .spawn_bundle(NodeBundle {
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             position: UiRect::all(Val::Px(0.)),
    //             flex_direction: FlexDirection::ColumnReverse,
    //             align_content: AlignContent::FlexStart,
    //             align_items: AlignItems::FlexStart,
    //             align_self: AlignSelf::FlexStart,
    //             justify_content: JustifyContent::FlexStart,
    //             ..Default::default()
    //         },
    //         color: Color::NONE.into(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         // match result string
    //         parent.spawn_bundle(TextBundle {
    //             text: Text::from_section(
    //                 "Active Zombies: 0",
    //                 TextStyle {
    //                     font_size: 50.,
    //                     font: fonts.fira_sans.clone(),
    //                     color: Color::rgba(1.0, 1.0, 1.0, 0.2),
    //                 },
    //             ),
    //             ..Default::default()
    //         });
    //     })
    //     .insert(RoundEntity);
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
            text.sections[1].value = player.1.active_zombies.to_string();
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
