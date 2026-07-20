use bevy::prelude::*;

use crate::game::snake::components::{Direction, Snake};

pub fn handle_input(mut snake_query: Query<&mut Snake>, keys: Res<ButtonInput<KeyCode>>) {
    for mut snake in snake_query.iter_mut() {
        for key in keys.get_just_pressed() {
            let new_direction = match key {
                KeyCode::KeyW | KeyCode::ArrowUp => Direction::Up,
                KeyCode::KeyS | KeyCode::ArrowDown => Direction::Down,
                KeyCode::KeyA | KeyCode::ArrowLeft => Direction::Left,
                KeyCode::KeyD | KeyCode::ArrowRight => Direction::Right,
                _ => continue,
            };

            if new_direction != snake.direction.opposite() {
                snake.next_direction = new_direction;
                break;
            }
        }
    }
}
