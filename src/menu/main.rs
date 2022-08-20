use crate::{menu::*, GGRSConfig, CHECK_DISTANCE, FPS, INPUT_DELAY, MAX_PREDICTION, NUM_PLAYERS};
use bevy::app::AppExit;
use bevy_ggrs::SessionType;
use ggrs::{PlayerType, SessionBuilder};

#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {
    OnlineMatch,
    LocalMatch,
    Quit,
}

pub fn setup_main_menu_ui(
    mut commands: Commands,
    image_assets: Res<TextureAssets>,
    font_assets: Res<FontAssets>,
) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::all(Val::Px(0.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // logo
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(500.0), Val::Px(139.0)),
                    margin: UiRect::all(Val::Px(16.)),
                    padding: UiRect::all(Val::Px(16.)),
                    ..Default::default()
                },
                image: image_assets.bevy_logo.clone().into(),
                ..Default::default()
            });

            // online match button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
                            "Online",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuMainBtn::OnlineMatch);

            // local mode button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
                            "Local",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuMainBtn::LocalMatch);

            // quit button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
                            "Quit",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuMainBtn::Quit);
        })
        .insert(MenuMainUI);
}

pub fn btn_listeners(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<(&Interaction, &MenuMainBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuMainBtn::OnlineMatch => {
                    commands.insert_resource(NextState(AppState::MenuOnline));
                }
                MenuMainBtn::LocalMatch => {
                    create_synctest_session(&mut commands);
                    commands.insert_resource(NextState(AppState::RoundLocal));
                }
                MenuMainBtn::Quit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}

fn create_synctest_session(commands: &mut Commands) {
    let mut sess_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(NUM_PLAYERS)
        .with_max_prediction_window(MAX_PREDICTION)
        .with_fps(FPS)
        .expect("Invalid FPS")
        .with_input_delay(INPUT_DELAY)
        .with_check_distance(CHECK_DISTANCE);

    for i in 0..NUM_PLAYERS {
        sess_build = sess_build.add_player(PlayerType::Local, i).expect("Could not add local player");
    }

    let sess = sess_build.start_synctest_session().expect("");

    commands.insert_resource(sess);
    commands.insert_resource(SessionType::SyncTestSession);
    commands.insert_resource(LocalHandles { handles: (0..NUM_PLAYERS).collect() });
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MenuMain, setup_main_menu_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MenuMain)
                    .with_system(btn_visuals::<MenuMainBtn>)
                    .with_system(btn_listeners)
                    .into(),
            )
            .add_exit_system(AppState::MenuMain, despawn_all_with::<MenuMainUI>);
    }
}
