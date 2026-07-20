use bevy::prelude::*;

use crate::game::ui::components::{MenuButtonAction, MenuRoot};
use crate::game::ui::resources::UiFont;
use crate::game::ui::systems::buttons::spawn_menu_button;
use crate::game::ui::systems::hints::spawn_hints;

pub fn spawn_menu(mut commands: Commands, font: Res<UiFont>) {
    commands
        .spawn((
            MenuRoot,
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("SNAKE"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_menu_button(root, MenuButtonAction::Start, "Start");
            spawn_hints(root, &font.0);
        });
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
