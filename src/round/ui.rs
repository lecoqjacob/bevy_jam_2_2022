use crate::round::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct ZombieText;

#[derive(Component)]
pub struct PlayerText;

#[derive(Component)]
pub struct RespawnText;

pub fn setup_round_ui(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Captured Zombies: ",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                    },
                ),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect { top: Val::Px(5.0), left: Val::Px(15.0), ..default() },
                ..default()
            }),
        )
        .insert(ZombieText);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Captured Zombies: ",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::WHITE,
                        font: fonts.fira_sans.clone(),
                    },
                ),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect { top: Val::Px(5.0), left: Val::Percent(30.), ..default() },
                ..default()
            }),
        )
        .insert(ZombieText);

    // root node
    // commands
    //     .spawn_bundle(NodeBundle {
    //         style: Style {
    //             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //             justify_content: JustifyContent::SpaceBetween,
    //             align_items: AlignItems::Center,
    //             align_content: AlignContent::Center,
    //             ..default()
    //         },
    //         color: Color::NONE.into(),
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent
    //             .spawn_bundle(
    //                 // Create a TextBundle that has a Text with a list of sections.
    //                 TextBundle::from_sections([
    //                     TextSection::new(
    //                         "Captured Zombies: ",
    //                         TextStyle {
    //                             font_size: 15.0,
    //                             color: Color::WHITE,
    //                             font: fonts.fira_sans.clone(),
    //                         },
    //                     ),
    //                     TextSection::new(
    //                         "0",
    //                         TextStyle {
    //                             font_size: 15.0,
    //                             color: Color::GOLD,
    //                             font: fonts.fira_sans.clone(),
    //                         },
    //                     ),
    //                 ])
    //                 .with_style(Style {
    //                     size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //                     justify_content: JustifyContent::SpaceBetween,
    //                     align_content: AlignContent::Center,
    //                     align_items: AlignItems::Center,
    //                     ..default()
    //                 })
    //                 .with_text_alignment(TextAlignment::CENTER),
    //             )
    //             .insert(ZombieText)
    //             .insert(RoundEntity);

    //         parent
    //             .spawn_bundle(
    //                 // Create a TextBundle that has a Text with a list of sections.
    //                 TextBundle::from_sections([TextSection::new(
    //                     "Player: ",
    //                     TextStyle {
    //                         font_size: 25.0,
    //                         color: Color::WHITE,
    //                         font: fonts.fira_sans.clone(),
    //                     },
    //                 )])
    //                 .with_style(Style {
    //                     align_self: AlignSelf::FlexEnd,
    //                     margin: UiRect { left: Val::Auto, right: Val::Auto, ..default() },
    //                     ..default()
    //                 }),
    //             )
    //             .insert(PlayerText)
    //             .insert(RoundEntity);
    //     });

    // commands
    //     .spawn_bundle(
    //         // Create a TextBundle that has a Text with a list of sections.
    //         TextBundle::from_sections([
    //             TextSection::new(
    //                 "Respawning in ",
    //                 TextStyle {
    //                     font_size: 40.0,
    //                     color: Color::WHITE,
    //                     font: fonts.fira_sans.clone(),
    //                 },
    //             ),
    //             TextSection::new(
    //                 "0",
    //                 TextStyle {
    //                     font_size: 40.0,
    //                     color: Color::GOLD,
    //                     font: fonts.fira_sans.clone(),
    //                 },
    //             ),
    //             TextSection::new(
    //                 " seconds",
    //                 TextStyle {
    //                     font_size: 40.0,
    //                     color: Color::WHITE,
    //                     font: fonts.fira_sans.clone(),
    //                 },
    //             ),
    //         ])
    //         .with_style(Style {
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 bottom: Val::Percent(50.),
    //                 left: Val::Percent(35.),
    //                 ..Default::default()
    //             },
    //             align_content: AlignContent::Center,
    //             ..default()
    //         }),
    //     )
    //     .insert(RespawnText)
    //     .insert(RoundEntity);
}

fn update_round_text(
    players: Query<&Player>,
    mut query: Query<&mut Text, (With<ZombieText>, Without<PlayerText>)>,
    mut player_text: Query<&mut Text, (With<PlayerText>, Without<ZombieText>)>,
) {
    // let player = players.iter().find(|p| p.handle == local_handle);

    // for mut text in &mut query {
    //     if let Some(player) = player {
    //         text.sections[1].value = player.active_zombies.len().to_string();
    //     }
    // }

    // for mut player_text in &mut player_text {
    //     if let Some(player) = player {
    //         player_text.sections[0].value = format!("Player {}", player.handle);
    //     }
    // }
}

////////////////////////////////////////////////////////////////////////////////
// FPS
////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
struct FPSText;

fn fps_text_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 15.0,
                            color: Color::GREEN,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FPSText);
}

fn fps_text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.0}", average);
                text.sections[1].style.color = if average > 60.0 {
                    Color::GREEN
                } else if average > 30.0 {
                    Color::YELLOW
                } else {
                    Color::RED
                };
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Respawn
////////////////////////////////////////////////////////////////////////////////

fn update_respawn_text(respawns: Query<&Respawn>, mut query: Query<&mut Text, With<RespawnText>>) {
    // let handle = local_handles.handles[0];
    // if let Some(r) = respawns.iter().find(|r| r.handle == handle) {
    //     for mut text in query.iter_mut() {
    //         text.sections.iter_mut().for_each(|section| {
    //             section.style.color.set_a(1.0);
    //         });

    //         text.sections[1].value = format!("{:.00}", r.time);
    //     }
    // } else {
    //     for mut text in query.iter_mut() {
    //         text.sections.iter_mut().for_each(|section| {
    //             section.style.color.set_a(0.0);
    //         });
    //     }
    // }
}

pub struct RoundUIPlugin;
impl Plugin for RoundUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());

        app.add_enter_system_set(
            AppState::InGame,
            ConditionSet::new().with_system(setup_round_ui).with_system(fps_text_setup).into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(update_round_text)
                .with_system(fps_text_update_system)
                .with_system(update_respawn_text)
                .into(),
        );
    }
}
