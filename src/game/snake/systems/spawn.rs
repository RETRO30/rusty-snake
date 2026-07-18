use bevy::prelude::*;
use std::collections::VecDeque;

use crate::game::grid::components::GridPosition;
use crate::game::grid::systems::base_sprites::get_square;
use crate::game::snake::components::{Direction, Snake, SnakeSegment, SnakeSegmentRole};
use crate::game::snake::constants::SNAKE_START_LENGTH;

pub fn spawn_snake(mut commands: Commands) {
    let mut segments: VecDeque<Entity> = VecDeque::new();
    let direction = Direction::Up;
    let start_position = GridPosition::new(0, 0);

    for i in 0..SNAKE_START_LENGTH {
        let role = if i == 0 {
            SnakeSegmentRole::Head
        } else if i == (SNAKE_START_LENGTH - 1) {
            SnakeSegmentRole::Tail
        } else {
            SnakeSegmentRole::Body
        };

        let position = start_position - (direction.to_grid() * i);
        segments.push_back(spawn_segment(&mut commands, position, role));
    }

    commands.spawn(Snake {
        segments,
        direction,
        length: SNAKE_START_LENGTH as usize,
        ..Snake::new(direction, SNAKE_START_LENGTH as usize)
    });
}

pub fn spawn_segment(
    commands: &mut Commands,
    position: GridPosition,
    role: SnakeSegmentRole,
) -> Entity {
    let sprite = get_square(Color::WHITE);

    commands
        .spawn((SnakeSegment, position, sprite, role, Transform::default()))
        .id()
}
