use bevy::prelude::*;

use crate::config::{CELL_SIZE, GRID_SIZE};
use crate::game::grid::systems::base_sprites::get_square;

pub fn spawn_board(mut commands: Commands) {
    let half_grid = (GRID_SIZE / 2) as i32;

    for x in -half_grid..half_grid {
        for y in -half_grid..half_grid {
            let color = if (x + y).rem_euclid(2) == 0 {
                Color::srgb(0.09, 0.09, 0.11)
            } else {
                Color::srgb(0.13, 0.13, 0.16)
            };

            commands.spawn((
                get_square(color),
                Transform::from_translation(Vec3::new(
                    x as f32 * CELL_SIZE as f32,
                    y as f32 * CELL_SIZE as f32,
                    -1.0,
                )),
            ));
        }
    }
}
