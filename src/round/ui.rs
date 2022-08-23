use crate::round::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_egui::{egui, EguiContext, EguiPlugin};

////////////////////////////////////////////////////////////////////////////////
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
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
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
// EGUI
////////////////////////////////////////////////////////////////////////////////
fn factors_system(
    // mut commands: Commands,
    // local_handles: Res<LocalHandles>,
    mut egui_context: ResMut<EguiContext>,
    // players: Query<(Entity, &Player)>,
    mut creatures: Query<(Entity, &mut CreatureType, &mut CreatureFollow, &mut CreatureSize)>,
) {
    egui::Window::new("Edit Factors")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -10.0])
        .vscroll(true)
        .show(egui_context.ctx_mut(), |ui| {
            for (entity, c_type, mut c_follow, mut c_size) in &mut creatures {
                ui.collapsing(
                    format!("Entity: ({:?}): Following: {:?}", entity, c_type.0.unwrap()),
                    |ui| {
                        ui.add(egui::Slider::new(&mut c_follow.0, 0.0..=100.0).text("Distance"));
                        ui.add(egui::Slider::new(&mut c_size.0, 0.0..=100.0).text("Size"));
                    },
                );
            }
        });
}

////////////////////////////////////////////////////////////////////////////////

pub struct RoundUIPlugin;
impl Plugin for RoundUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_plugin(FrameTimeDiagnosticsPlugin::default());

        app.add_enter_system_set(
            AppState::RoundLocal,
            ConditionSet::new().with_system(setup_round_ui).with_system(fps_text_setup).into(),
        );
        app.add_enter_system_set(
            AppState::RoundOnline,
            ConditionSet::new().with_system(setup_round_ui).with_system(fps_text_setup).into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundLocal)
                .with_system(update_round_text)
                .with_system(fps_text_update_system)
                .with_system(factors_system)
                .into(),
        );
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::RoundOnline)
                .with_system(update_round_text)
                .with_system(fps_text_update_system)
                .with_system(factors_system)
                .into(),
        );
    }
}
