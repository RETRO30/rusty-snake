use derive_more::{Add, Deref, DerefMut, Sub};
use std::ops::Mul;

use crate::config::CELL_SIZE;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Deref, DerefMut, Add, Sub)]
pub struct GridPosition(IVec3);

impl GridPosition {
    pub fn fromxy(x: i32, y: i32) -> Self {
        GridPosition(IVec3::new(x, y, 0))
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            (self.0.x * CELL_SIZE) as f32,
            (self.0.y * CELL_SIZE) as f32,
            self.0.z as f32,
        )
    }
}

impl Mul<i32> for GridPosition {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        GridPosition(self.0 * rhs)
    }
}
