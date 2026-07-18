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
            Direction::Up => GridPosition::new(0, 1),
            Direction::Down => GridPosition::new(0, -1),
            Direction::Left => GridPosition::new(-1, 0),
            Direction::Right => GridPosition::new(1, 0),
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
    pub direction: Direction,
    pub length: usize,
    pub movment_timer: Timer,
}

impl Snake {
    pub fn new(direction: Direction, length: usize) -> Self {
        Self {
            segments: VecDeque::new(),
            direction,
            length,
            movment_timer: Timer::new(
                Duration::from_millis(MOVEMENT_DELAY_MILLIS),
                TimerMode::Repeating,
            ),
        }
    }
}
