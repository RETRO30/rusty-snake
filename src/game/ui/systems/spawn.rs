use bevy::prelude::*;

use crate::config::{BORDER_THICKNESS, PLAYFIELD_PX, TOP_BAR_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::ui::components::{MenuButtonAction, ScoreText};

pub fn spawn_score_ui(mut commands: Commands) {
    // Position the score bar in percentages of the canvas so it lines up with
    // the playfield frame at any canvas size. This relies on the canvas keeping
    // the game's aspect ratio (`#game-wrap` in web/index.html), so window
    // fractions map 1:1 to the world fractions the camera renders.
    let left_pct = BORDER_THICKNESS / WINDOW_WIDTH * 100.0;
    let width_pct = PLAYFIELD_PX / WINDOW_WIDTH * 100.0;
    let bar_vh = TOP_BAR_HEIGHT / WINDOW_HEIGHT * 100.0;
    // ~24px tall at the native height, scaling with the canvas.
    let font_vh = bar_vh * 0.45;

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Percent(left_pct),
            width: Val::Percent(width_pct),
            height: Val::Vh(bar_vh),
            align_items: AlignItems::Center,
            // Score on the left, pause button pinned to the right edge of the frame.
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::horizontal(Val::Px(12.0)),
            ..default()
        })
        .with_children(|bar| {
            bar.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font_size: FontSize::Vh(font_vh),
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));

            // Pause button — a compact square so touch users (no Esc key) can
            // pause. `handle_menu_buttons` only acts on it while playing.
            bar.spawn((
                Button,
                MenuButtonAction::Pause,
                Node {
                    height: Val::Percent(72.0),
                    aspect_ratio: Some(1.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(Color::WHITE),
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("II"),
                    TextFont {
                        font_size: FontSize::Vh(font_vh),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}
