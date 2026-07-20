use bevy::prelude::*;

use super::spawn::spawn_segment;
use crate::game::grid::components::GridPosition;
use crate::game::snake::components::{Snake, SnakeSegmentRole};

fn set_segment_role(
    role_query: &mut Query<&mut SnakeSegmentRole>,
    entity: Entity,
    new_role: SnakeSegmentRole,
) {
    if let Ok(mut role) = role_query.get_mut(entity) {
        *role = new_role;
    } else {
        warn!("Segment {entity:?} has no SnakeSegmentRole")
    }
}

pub fn move_snake(
    mut commands: Commands,
    mut snake_query: Query<&mut Snake>,
    mut role_query: Query<&mut SnakeSegmentRole>,
    mut grid_query: Query<&mut GridPosition>,
    time: Res<Time>,
) {
    for mut snake in snake_query.iter_mut() {
        snake.movment_timer.tick(time.delta());

        if !snake.movment_timer.just_finished() {
            continue;
        }

        snake.direction = snake.next_direction;

        let Some(head) = snake.segments.front() else {
            warn!("Snake is empty, cannot move");
            continue;
        };
        let Ok(position) = grid_query.get(*head) else {
            warn!("Head doesn't have position");
            continue;
        };

        set_segment_role(&mut role_query, *head, SnakeSegmentRole::Body);

        let new_head_position = *position + snake.direction.to_grid();

        let new_head: Entity = if snake.length <= snake.segments.len() {
            let Some(tail) = snake.segments.pop_back() else {
                warn!("Snake is empty, cannot move");
                continue;
            };
            let Ok(mut segment_position) = grid_query.get_mut(tail) else {
                warn!("Segment doesn't have position");
                continue;
            };
            *segment_position = new_head_position;
            set_segment_role(&mut role_query, tail, SnakeSegmentRole::Head);
            tail
        } else {
            spawn_segment(&mut commands, new_head_position, SnakeSegmentRole::Head)
        };

        snake.segments.push_front(new_head);

        let Some(new_tail) = snake.segments.back() else {
            warn!("Snake is empty, cannot change role");
            continue;
        };
        set_segment_role(&mut role_query, *new_tail, SnakeSegmentRole::Tail);
    }
}
