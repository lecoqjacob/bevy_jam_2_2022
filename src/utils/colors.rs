// Add Custom Colors :)

use bevy::prelude::Color;

pub const RED: Color = Color::TOMATO;
pub const BLUE: Color = Color::BLUE;
pub const PURPLE: Color = Color::PURPLE;
pub const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);

pub fn get_color_name(color: Color) -> &'static str {
    if color == BLUE {
        return "Blue";
    } else if color == RED {
        return "Red";
    } else if color == PURPLE {
        return "Purple";
    } else if color == GREEN {
        return "Green";
    }

    unreachable!("Should never get here")
}
