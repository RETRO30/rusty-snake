use bevy::prelude::*;

use crate::game::food::components::Food;
use crate::game::grid::components::GridPosition;
use crate::game::snake::components::{Snake, SnakeSegment};
use crate::game::ui::resources::Score;

pub fn eat_food(
    mut commands: Commands,
    mut snake_query: Query<&mut Snake>,
    segment_positions: Query<&GridPosition, With<SnakeSegment>>,
    food_query: Query<(Entity, &GridPosition), With<Food>>,
    mut score: ResMut<Score>,
) {
    for mut snake in &mut snake_query {
        let Some(head) = snake.segments.front() else {
            continue;
        };
        let Ok(head_position) = segment_positions.get(*head) else {
            continue;
        };

        for (food_entity, food_position) in &food_query {
            if food_position == head_position {
                commands.entity(food_entity).despawn();
                snake.length += 1;
                score.0 += 1;
                break;
            }
        }
    }
}
