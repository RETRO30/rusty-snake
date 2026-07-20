use bevy::prelude::*;

use crate::game::state::GameState;

use super::systems::{
    animation::sync_scale_by_role, collision::check_collisions, input::handle_input,
    movement::move_snake, touch::handle_swipe,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_input,
            handle_swipe,
            move_snake,
            check_collisions,
            sync_scale_by_role,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}
