use bevy::prelude::*;
use rand::RngExt;
use std::collections::HashSet;

use crate::config::GRID_SIZE;
use crate::game::food::components::Food;
use crate::game::grid::components::GridPosition;
use crate::game::grid::systems::base_sprites::get_square;
use crate::game::snake::components::SnakeSegment;

pub fn spawn_food(commands: &mut Commands, position: GridPosition) {
    let sprite = get_square(Color::srgb(255., 0., 0.));

    commands.spawn((Food, position, sprite, Transform::default()));
}

pub fn spawn_food_on_free_space(
    mut commands: Commands,
    food_query: Query<(), With<Food>>,
    segments_query: Query<&GridPosition, With<SnakeSegment>>,
) {
    if !food_query.is_empty() {
        return;
    }

    let occupied: HashSet<GridPosition> = segments_query.iter().copied().collect();
    let half_grid = (GRID_SIZE / 2) as i32;

    let mut rng = rand::rng();
    let free_position = (0..GRID_SIZE * GRID_SIZE)
        .map(|_| {
            GridPosition::new(
                rng.random_range(-half_grid..half_grid),
                rng.random_range(-half_grid..half_grid),
            )
        })
        .find(|position| !occupied.contains(position));

    match free_position {
        Some(position) => spawn_food(&mut commands, position),
        None => warn!("No free space to spawn food"),
    }
}
