// Add Custom Colors :)

use bevy::prelude::Color;

pub const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
pub const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
pub const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
pub const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);

pub fn get_color_name(color: Color) -> &'static str {
    if color == BLUE {
        "Blue"
    } else if color == ORANGE {
        "Orange"
    } else if color == MAGENTA {
        "Magenta"
    } else {
        "Green"
    }
}
