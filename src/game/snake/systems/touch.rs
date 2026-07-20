use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::snake::components::{Direction, Snake};

/// Minimum drag distance (logical/CSS pixels) before a swipe registers a
/// turn. Once a swipe fires, the origin resets to the current touch
/// position, so one continuous drag can chain multiple turns (e.g. an
/// L-shaped swipe turns right, then down) instead of needing a fresh touch
/// per turn — and turns land immediately instead of waiting for the finger
/// to lift, which is what made mobile input feel laggy.
const MIN_SWIPE_DISTANCE: f32 = 24.0;

pub fn handle_swipe(
    mut snake_query: Query<&mut Snake>,
    touches: Res<Touches>,
    mut origins: Local<HashMap<u64, Vec2>>,
) {
    origins.retain(|id, _| touches.get_pressed(*id).is_some());

    for touch in touches.iter() {
        let origin = *origins
            .entry(touch.id())
            .or_insert_with(|| touch.start_position());
        let delta = touch.position() - origin;

        if delta.length() < MIN_SWIPE_DISTANCE {
            continue;
        }

        let new_direction = if delta.x.abs() > delta.y.abs() {
            if delta.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if delta.y > 0.0 {
            Direction::Down
        } else {
            Direction::Up
        };

        origins.insert(touch.id(), touch.position());

        for mut snake in &mut snake_query {
            if new_direction != snake.direction.opposite() {
                snake.next_direction = new_direction;
            }
        }
    }
}
