use crate::round::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct P1ZombieText;

#[derive(Component)]
pub struct P2ZombieText;

#[derive(Component)]
pub struct P1RespawnText;

#[derive(Component)]
pub struct P2RespawnText;

#[derive(Component)]
pub struct Indicator;

pub fn setup_round_ui(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<TextureAssets>,
) {
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
        .insert(P1ZombieText)
        .insert(RoundEntity);

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
        .insert(P2ZombieText)
        .insert(RoundEntity);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Respawning in ",
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
                TextSection::new(
                    " seconds",
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
                position: UiRect { top: Val::Percent(25.0), left: Val::Percent(5.), ..default() },
                ..default()
            }),
        )
        .insert(P1RespawnText)
        .insert(RoundEntity);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Respawning in ",
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
                TextSection::new(
                    " seconds",
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
                position: UiRect { top: Val::Percent(25.0), left: Val::Percent(30.), ..default() },
                ..default()
            }),
        )
        .insert(P2RespawnText)
        .insert(RoundEntity);

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(30.0), Val::Auto),
                // This takes the icons out of the flexbox flow, to be positioned exactly
                position_type: PositionType::Absolute,
                // The icon will be close to the left border of the button
                position: UiRect {
                    left: Val::Percent(35.),
                    top: Val::Px(25.),
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                ..default()
            },
            image: UiImage(textures.arrow.clone()),
            ..default()
        })
        .insert(Indicator)
        .insert(SnapToPlayer(0))
        .insert(RoundEntity);

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(30.0), Val::Auto),
                // This takes the icons out of the flexbox flow, to be positioned exactly
                position_type: PositionType::Absolute,
                // The icon will be close to the left border of the button
                position: UiRect {
                    left: Val::Percent(10.),
                    top: Val::Px(25.),
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                ..default()
            },
            image: UiImage(textures.arrow.clone()),
            ..default()
        })
        .insert(Indicator)
        .insert(SnapToPlayer(1))
        .insert(RoundEntity);
}

fn update_round_text(
    players: Query<&Player>,
    mut texts: ParamSet<(
        Query<&mut Text, With<P1ZombieText>>,
        Query<&mut Text, With<P2ZombieText>>,
    )>,
) {
    let players = players.iter().collect::<Vec<_>>();
    if let Some(p1) = players.get(0) {
        for mut text in &mut texts.p0() {
            text.sections[1].value = p1.active_zombies.len().to_string();
        }
    }
    if let Some(p2) = players.get(1) {
        for mut text in &mut texts.p1() {
            text.sections[1].value = p2.active_zombies.len().to_string();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// FPS
////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
struct FPSText;

fn fps_text_setup(mut commands: Commands, fonts: Res<FontAssets>) {
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
                            font: fonts.fira_sans.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: fonts.fira_sans.clone(),
                            font_size: 15.0,
                            color: Color::GREEN,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FPSText)
        .insert(RoundEntity);
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

fn update_respawn_text(
    respawns: Query<(&Player, &Clock), (With<Player>, With<Dead>, Changed<Clock>)>,
    mut evs: EventReader<SpawnEvent>,
    mut texts: ParamSet<(
        Query<&mut Text, With<P1RespawnText>>,
        Query<&mut Text, With<P2RespawnText>>,
    )>,
) {
    if respawns.is_empty() {
        texts.p0().iter_mut().for_each(|mut text| {
            text.sections.iter_mut().for_each(|section| {
                section.style.color.set_a(0.0);
            });
        });
        texts.p1().iter_mut().for_each(|mut text| {
            text.sections.iter_mut().for_each(|section| {
                section.style.color.set_a(0.0);
            });
        });
    }

    for ev in evs.iter() {
        if ev.spawn_type == SpawnType::Player {
            if ev.handle.unwrap() == 0 {
                for mut text in texts.p0().iter_mut() {
                    text.sections.iter_mut().for_each(|sec| {
                        sec.style.color.set_a(0.0);
                    });
                }
            } else {
                for mut text in texts.p1().iter_mut() {
                    text.sections.iter_mut().for_each(|sec| {
                        sec.style.color.set_a(0.0);
                    });
                }
            }
        }
    }

    for (p, c) in respawns.iter() {
        if p.handle == 0 {
            for mut text in texts.p1().iter_mut() {
                text.sections.iter_mut().for_each(|sec| {
                    sec.style.color.set_a(1.0);
                });
                text.sections[1].value = format!("{:.2}", c.current);
            }
        } else {
            for mut text in texts.p1().iter_mut() {
                text.sections.iter_mut().for_each(|sec| {
                    sec.style.color.set_a(1.0);
                });
                text.sections[1].value = format!("{:.2}", c.current);
            }
        }
    }
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
