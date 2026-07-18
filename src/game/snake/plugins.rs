use bevy::prelude::*;

use super::systems::{
    animation::sync_scale_by_role, input::handle_input, movement::move_snake, spawn::spawn_snake,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_snake);
    app.add_systems(
        Update,
        (handle_input, move_snake, sync_scale_by_role).chain(),
    );
}
