use bevy::math::curve::{EaseFunction, EasingCurve};
use bevy::prelude::*;

use crate::game::snake::components::{Snake, SnakeSegmentRole};
use crate::game::snake::constants::BODY_SCALE;

pub fn sync_scale_by_role(
    snakes: Query<&Snake>,
    mut segments: Query<(&SnakeSegmentRole, &mut Transform)>,
) {
    for snake in &snakes {
        let fraction = snake.movment_timer.fraction();

        let head_curve = EasingCurve::new(0., BODY_SCALE, EaseFunction::QuinticInOut);
        let tail_curve = EasingCurve::new(BODY_SCALE, 0., EaseFunction::QuinticInOut);
        let base_scale = Vec3::new(1., 1., 0.);

        for (role, mut transform) in &mut segments {
            let new_scale = match role {
                SnakeSegmentRole::Head => base_scale * head_curve.sample_clamped(fraction),
                SnakeSegmentRole::Tail => base_scale * tail_curve.sample_clamped(fraction),
                SnakeSegmentRole::Body => base_scale * BODY_SCALE,
            };
            if new_scale != transform.scale {
                transform.scale = new_scale;
            }
        }
    }
}
