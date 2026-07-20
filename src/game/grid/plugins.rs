use crate::game::grid::systems::spawn_frame::spawn_frame;
use crate::game::grid::systems::sync::sync_grid_to_transform;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_frame);
    app.add_systems(Update, sync_grid_to_transform);
}
