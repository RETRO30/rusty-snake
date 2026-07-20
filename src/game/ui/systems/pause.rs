use bevy::prelude::*;

use crate::game::ui::components::{MenuButtonAction, PauseRoot};
use crate::game::ui::resources::UiFont;
use crate::game::ui::systems::buttons::spawn_menu_button;
use crate::game::ui::systems::hints::spawn_hints;

pub fn spawn_pause(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            PauseRoot,
            GlobalZIndex(10),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Paused"),
                TextFont {
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                // Wrap to a second row instead of overflowing off-screen on
                // narrow phones, where two 180px buttons + gap don't fit.
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                max_width: Val::Vw(90.0),
                column_gap: Val::Px(16.0),
                row_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|row| {
                spawn_menu_button(row, MenuButtonAction::Resume, "Resume");
                spawn_menu_button(row, MenuButtonAction::Restart, "Restart");
            });

            spawn_hints(root, &font.0);
        });
}

pub fn despawn_pause(mut commands: Commands, query: Query<Entity, With<PauseRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
