use bevy::render::camera::Viewport;

use crate::menu::*;

#[derive(Component)]
pub struct WinUI;

#[derive(Component)]
pub enum MenuWinBtn {
    Back,
}

pub struct MatchData {
    pub result: String,
}

pub fn setup_win_ui(
    windows: Res<Windows>,
    mut commands: Commands,
    match_data: Res<MatchData>,
    font_assets: Res<FontAssets>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
) {
    let window = windows.primary();
    let mut left_camera = left_camera.single_mut();
    left_camera.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(window.physical_width(), window.physical_height()),
        ..default()
    });

    let mut right_camera = right_camera.single_mut();
    right_camera.viewport = None;

    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // match result string
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    match_data.result.clone(),
                    TextStyle {
                        font: font_assets.fira_sans.clone(),
                        font_size: 50.,
                        color: BUTTON_TEXT,
                    },
                ),
                ..Default::default()
            });
            // back to menu button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(16.)),
                        padding: UiRect::all(Val::Px(16.)),
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Back to Menu",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuWinBtn::Back);
        })
        .insert(WinUI);

    commands.remove_resource::<MatchData>();
}

pub fn btn_listeners(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &MenuWinBtn), Changed<Interaction>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuWinBtn::Back => {
                    commands.insert_resource(NextState(AppState::MenuMain));
                    audio.play(audio_assets.click.clone());
                }
            }
        }
    }
}

pub struct WinMenuPlugin;
impl Plugin for WinMenuPlugin {
    fn build(&self, app: &mut App) {
        // win menu
        app.add_enter_system(AppState::Win, setup_win_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Win)
                    .with_system(btn_visuals::<MenuWinBtn>)
                    .with_system(btn_listeners)
                    .into(),
            )
            .add_exit_system(AppState::Win, despawn_all_with::<WinUI>);
    }
}
