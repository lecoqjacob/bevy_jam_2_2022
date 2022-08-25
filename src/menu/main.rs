use crate::menu::*;
use bevy::app::AppExit;

#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {
    PlayGame,
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
                            "Play Game",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: BUTTON_TEXT,
                            },
                        ),
                        ..Default::default()
                    });
                })
                .insert(MenuMainBtn::PlayGame);

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
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<(&Interaction, &MenuMainBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuMainBtn::PlayGame => {
                    commands.insert_resource(NextState(AppState::WorldGen));
                }
                MenuMainBtn::Quit => {
                    exit.send(AppExit);
                }
            }
        }
    }

    commands.insert_resource(NextState(AppState::WorldGen));
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
