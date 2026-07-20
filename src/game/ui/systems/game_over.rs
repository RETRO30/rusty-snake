use bevy::prelude::*;

use crate::game::ui::components::{GameOverRoot, MenuButtonAction};
use crate::game::ui::resources::Score;
use crate::game::ui::systems::buttons::spawn_menu_button;

pub fn spawn_game_over(mut commands: Commands, score: Res<Score>) {
    let final_score = score.0;

    commands
        .spawn((
            GameOverRoot,
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
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: FontSize::Px(44.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            root.spawn((
                Text::new(format!("Final score: {final_score}")),
                TextFont {
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_menu_button(root, MenuButtonAction::Restart, "Restart");
        });
}

pub fn despawn_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
