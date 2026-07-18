use bevy::prelude::*;

use super::food::plugins as food_plugins;
use super::grid::plugins as grid_plugins;
use super::snake::plugins as snake_plugins;
use super::ui::plugins as ui_plugins;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        grid_plugins::plugin,
        snake_plugins::plugin,
        food_plugins::plugin,
        ui_plugins::plugin,
    ));
}
