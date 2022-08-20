pub mod connect;
use crate::prelude::*;

pub mod main;
pub mod online;
pub mod win;

use bevy::prelude::PluginGroup;
pub use connect::*;
pub use main::*;
pub use online::*;
pub use win::*;

const DISABLED_BUTTON: Color = Color::rgb(0.8, 0.5, 0.5);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

pub struct MenuPlugins;
impl PluginGroup for MenuPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(ConnectPlugin);
    }
}
