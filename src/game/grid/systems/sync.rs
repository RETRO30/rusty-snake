use bevy::prelude::*;

use crate::game::grid::components::GridPosition;

pub fn sync_grid_to_transform(
    mut query: Query<(&GridPosition, &mut Transform), Changed<GridPosition>>,
) {
    for (grid_pos, mut transform) in &mut query {
        transform.translation = grid_pos.to_vec3();
    }
}
