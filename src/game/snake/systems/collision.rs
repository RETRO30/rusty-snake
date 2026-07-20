use bevy::prelude::*;

use crate::config::GRID_SIZE;
use crate::game::grid::components::GridPosition;
use crate::game::snake::components::{Snake, SnakeSegment};
use crate::game::state::GameState;

pub fn check_collisions(
    snake_query: Query<&Snake>,
    segment_positions: Query<&GridPosition, With<SnakeSegment>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let half_grid = (GRID_SIZE / 2) as i32;

    for snake in &snake_query {
        let Some(head) = snake.segments.front() else {
            continue;
        };
        let Ok(head_position) = segment_positions.get(*head) else {
            continue;
        };
        let head_pos: IVec3 = **head_position;

        let out_of_bounds = head_pos.x < -half_grid
            || head_pos.x >= half_grid
            || head_pos.y < -half_grid
            || head_pos.y >= half_grid;

        if out_of_bounds {
            next_state.set(GameState::GameOver);
            return;
        }

        let self_collision = snake
            .segments
            .iter()
            .skip(1)
            .filter_map(|entity| segment_positions.get(*entity).ok())
            .any(|position| position == head_position);

        if self_collision {
            next_state.set(GameState::GameOver);
            return;
        }
    }
}
