use bevy::prelude::*;

use crate::config::{BORDER_THICKNESS, PLAYFIELD_PX};

const BORDER_COLOR: Color = Color::srgb(1., 1., 1.);

pub fn spawn_frame(mut commands: Commands) {
    let half_playfield = PLAYFIELD_PX / 2.0;
    let edge_offset = half_playfield + BORDER_THICKNESS / 2.0;

    let horizontal_size = Vec2::new(PLAYFIELD_PX + BORDER_THICKNESS * 2.0, BORDER_THICKNESS);
    let vertical_size = Vec2::new(BORDER_THICKNESS, PLAYFIELD_PX);

    let edges = [
        (Vec3::new(0.0, edge_offset, 1.0), horizontal_size),
        (Vec3::new(0.0, -edge_offset, 1.0), horizontal_size),
        (Vec3::new(edge_offset, 0.0, 1.0), vertical_size),
        (Vec3::new(-edge_offset, 0.0, 1.0), vertical_size),
    ];

    for (translation, size) in edges {
        commands.spawn((
            Sprite::from_color(BORDER_COLOR, size),
            Transform::from_translation(translation),
        ));
    }
}
