use bevy::prelude::*;

use crate::config::CELL_SIZE;

pub fn get_square(color: Color) -> Sprite {
    Sprite::from_color(color, Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32))
}
