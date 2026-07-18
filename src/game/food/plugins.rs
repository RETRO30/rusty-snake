use bevy::prelude::*;

use crate::game::snake::systems::movement::move_snake;

use super::systems::eat::eat_food;
use super::systems::spawn::spawn_food_on_free_space;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_food_on_free_space);
    app.add_systems(
        Update,
        (eat_food, spawn_food_on_free_space)
            .chain()
            .after(move_snake),
    );
}
