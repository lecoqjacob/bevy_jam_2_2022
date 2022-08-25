// This is the folder to handle all menu states

use crate::prelude::*;

pub mod main;
pub mod win;

pub use main::*;
pub use win::*;

const DISABLED_BUTTON: Color = Color::rgb(0.8, 0.5, 0.5);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

pub fn btn_visuals<BTN: Component>(
    mut interaction_query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<BTN>)>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub struct MenuPlugins;
impl PluginGroup for MenuPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(MainMenuPlugin).add(WinMenuPlugin);
    }
}
