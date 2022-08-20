use crate::menu::*;

#[derive(Component)]
pub struct MenuConnectUI;

#[derive(Component)]
pub enum MenuConnectBtn {
    Back,
}

pub fn setup_connect_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
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
        .insert(MenuConnectUI)
        .with_children(|parent| {
            // lobby id display
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                text: Text::from_section(
                    "Searching a match...",
                    TextStyle { font: font_assets.fira_sans.clone(), font_size: 32., color: BUTTON_TEXT },
                ),
                ..Default::default()
            });

            // back button
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
                .insert(MenuConnectBtn::Back);
        });
}

pub fn btn_listeners(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &MenuConnectBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuConnectBtn::Back => {
                    commands.insert_resource(NextState(AppState::MenuMain));
                }
            }
        }
    }
}

pub struct ConnectUIPlugin;
impl Plugin for ConnectUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MenuConnect, setup_connect_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MenuConnect)
                    .with_system(btn_visuals::<MenuConnectBtn>)
                    .with_system(btn_listeners)
                    .into(),
            )
            .add_exit_system(AppState::MenuConnect, despawn_all_with::<MenuConnectUI>);
    }
}
