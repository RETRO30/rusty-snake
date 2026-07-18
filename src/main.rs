use bevy::prelude::*;

mod config;
mod game;

use crate::config::{CELL_SIZE, GRID_SIZE};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Snake"),
                resolution: (CELL_SIZE * GRID_SIZE, CELL_SIZE * GRID_SIZE).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(game::plugins::plugin)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
