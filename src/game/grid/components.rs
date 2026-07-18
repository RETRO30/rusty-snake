use derive_more::{Add, Deref, DerefMut, Sub};
use std::ops::Mul;

use crate::config::CELL_SIZE;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Deref, DerefMut, Add, Sub)]
pub struct GridPosition(IVec2);

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition(IVec2::new(x, y))
    }
    pub fn to_vec3(&self) -> Vec3 {
        (self.0.as_vec2() * CELL_SIZE as f32).extend(0.0)
    }
}

impl Mul<i32> for GridPosition {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        GridPosition(self.0 * rhs)
    }
}
