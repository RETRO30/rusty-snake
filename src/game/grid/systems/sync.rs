use bevy::prelude::*;

use crate::config::CELL_SIZE;
use crate::game::grid::components::GridPosition;

pub fn sync_grid_to_transform(
    mut query: Query<(&GridPosition, &mut Transform), Changed<GridPosition>>,
) {
    for (grid_pos, mut transform) in &mut query {
        let vec_pos = grid_pos.to_vec3();

        transform.translation.x = vec_pos.x + CELL_SIZE as f32 / 2.0;
        transform.translation.y = vec_pos.y + CELL_SIZE as f32 / 2.0;
        transform.translation.z = vec_pos.z;
    }
}
