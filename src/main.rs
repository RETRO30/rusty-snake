use bevy::camera::ScalingMode;
use bevy::prelude::*;

mod config;
mod game;

use crate::config::{TOP_BAR_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Snake"),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                resizable: true,
                canvas: Some("#bevy-canvas".into()),
                // Track the size of the canvas's parent element (`#game-wrap` in
                // web/index.html, which caps the size and locks the aspect ratio)
                // instead of a fixed pixel size, so the game scales to the screen.
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(game::plugins::plugin)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // AutoMin guarantees the full playfield + top bar is always visible
        // without cropping, regardless of the window/canvas aspect ratio — on a
        // mismatched aspect ratio it shows extra background space on one axis
        // rather than cutting off the other.
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
        // Shift the camera up so the reserved bar strip sits above the frame
        // (the playfield is centered at the world origin); the score renders there.
        Transform::from_xyz(0.0, TOP_BAR_HEIGHT / 2.0, 0.0),
    ));
}
