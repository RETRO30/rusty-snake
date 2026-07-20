use bevy::prelude::*;
use std::{collections::VecDeque, time::Duration};

use crate::game::grid::components::GridPosition;
use crate::game::snake::constants::MOVEMENT_DELAY_MILLIS;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_grid(&self) -> GridPosition {
        match self {
            Direction::Up => GridPosition::fromxy(0, 1),
            Direction::Down => GridPosition::fromxy(0, -1),
            Direction::Left => GridPosition::fromxy(-1, 0),
            Direction::Right => GridPosition::fromxy(1, 0),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub enum SnakeSegmentRole {
    Head,
    Body,
    Tail,
}

#[derive(Component)]
pub struct Snake {
    pub segments: VecDeque<Entity>,
    /// Direction actually committed to the last (or current) movement tick.
    pub direction: Direction,
    /// Direction requested by input since the last tick. Input systems only
    /// ever validate against `direction` (not this field), so several inputs
    /// queued within one tick interval can't compound into a 180° reversal —
    /// `move_snake` commits at most one turn per tick.
    pub next_direction: Direction,
    pub length: usize,
    pub movment_timer: Timer,
}

impl Snake {
    pub fn new(direction: Direction, length: usize) -> Self {
        Self {
            segments: VecDeque::new(),
            direction,
            next_direction: direction,
            length,
            movment_timer: Timer::new(
                Duration::from_millis(MOVEMENT_DELAY_MILLIS),
                TimerMode::Repeating,
            ),
        }
    }
}
