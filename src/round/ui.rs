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

fn update_respawn_text(
    respawns: Query<&Clock, (With<Player>, With<Dead>)>,
    players: Query<Entity, With<Player>>,
    mut texts: ParamSet<(
        Query<&mut Text, With<P1RespawnText>>,
        Query<&mut Text, With<P2RespawnText>>,
    )>,
) {
    let players = players.iter().collect::<Vec<_>>();

    // Player 1
    if let Some(p1) = players.get(0) {
        if let Ok(c) = respawns.get(*p1) {
            for mut text in &mut texts.p0() {
                text.sections[1].value = format!("{:.2}", c.current);
            }
        }
    } else {
        for mut text in &mut texts.p0() {
            text.sections.iter_mut().for_each(|section| {
                section.style.color.set_a(0.0);
            });
        }
    }

    // Player 2
    if let Some(p2) = players.get(1) {
        if let Ok(c) = respawns.get(*p2) {
            for mut text in &mut texts.p1() {
                text.sections[1].value = format!("{:.2}", c.current);
            }
        }
    } else {
        for mut text in &mut texts.p1() {
            text.sections.iter_mut().for_each(|section| {
                section.style.color.set_a(0.0);
            });
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
