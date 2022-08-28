use crate::menu::*;

#[derive(Component)]
pub struct ControlsUI;

pub fn setup_controls_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
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
            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new(
                        "Welcome to Brain Hoarders!\n\n",
                        TextStyle {
                            font_size: 50.0,
                            color: Color::WHITE,
                            font: font_assets.fira_sans.clone(),
                        },
                    ),
                    TextSection::new(
                        "First to collect 50 brains wins!\n\n\n",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::WHITE,
                            font: font_assets.fira_sans.clone(),
                        },
                    ),
                ])
                .with_text_alignment(TextAlignment::CENTER),
            );

            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new(
                        "Player1:\n",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::WHITE,
                            font: font_assets.fira_sans.clone(),
                        },
                    ),
                    TextSection::new(
                        "Movement: WASD\nBoost: LShift\nFire: Space!\n\n",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::WHITE,
                            font: font_assets.fira_sans.clone(),
                        },
                    ),
                ])
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style { margin: UiRect::all(Val::Px(16.)), ..Default::default() }),
            );

            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new(
                        "Player2:\n",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::WHITE,
                            font: font_assets.fira_sans.clone(),
                        },
                    ),
                    TextSection::new(
                        "Movement: Arrow Keys\nBoost: B\nFire: M!\n",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::WHITE,
                            font: font_assets.fira_sans.clone(),
                        },
                    ),
                ])
                .with_text_alignment(TextAlignment::CENTER)
                .with_style(Style { margin: UiRect::all(Val::Px(16.)), ..Default::default() }),
            );
        })
        .insert(ControlsUI);
}

pub fn btn_listeners(
    audio: Res<Audio>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    audio_assets: Res<AudioAssets>,
) {
    if keys.just_pressed(KeyCode::Return) {
        audio.play(audio_assets.click.clone());
        commands.insert_resource(NextState(AppState::WorldGen));
    }
}

pub struct ControlsMenuPlugin;
impl Plugin for ControlsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::Controls, setup_controls_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Controls)
                    .with_system(btn_listeners)
                    .into(),
            )
            .add_exit_system(AppState::Controls, despawn_all_with::<ControlsUI>);
    }
}
