use crate::game::grid::systems::spawn_board::spawn_board;
use crate::game::grid::systems::sync::sync_grid_to_transform;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_board);
    app.add_systems(Update, sync_grid_to_transform);
}
