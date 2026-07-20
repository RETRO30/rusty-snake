use bevy::prelude::*;

use crate::game::food::components::Food;
use crate::game::snake::components::{Snake, SnakeSegment};
use crate::game::state::{self, GameState};
use crate::game::ui::components::MenuButtonAction;
use crate::game::ui::resources::Score;
const HOVER_SCALE: f32 = 1.05;
const PRESSED_SCALE: f32 = 0.95;
const DEFAULT_SCALE: f32 = 1.0;
const BUTTON_TEXT: Color = Color::WHITE;

pub fn spawn_menu_button(parent: &mut ChildSpawnerCommands, action: MenuButtonAction, label: &str) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                // border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BorderColor::all(BUTTON_TEXT),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(BUTTON_TEXT),
            ));
        });
}

pub fn handle_menu_buttons(
    mut commands: Commands,
    mut interactions: Query<
        (&Interaction, &MenuButtonAction, &mut UiTransform),
        Changed<Interaction>,
    >,
    snakes: Query<Entity, With<Snake>>,
    segments: Query<Entity, With<SnakeSegment>>,
    food: Query<Entity, With<Food>>,
    mut score: ResMut<Score>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action, mut transform) in &mut interactions {
        match interaction {
            Interaction::Pressed => {
                transform.scale = Vec2::splat(PRESSED_SCALE);
                match action {
                    MenuButtonAction::Start | MenuButtonAction::Restart => {
                        state::start_new_game(
                            &mut commands,
                            &snakes,
                            &segments,
                            &food,
                            &mut score,
                            &mut next_state,
                        );
                    }
                    MenuButtonAction::Resume => {
                        next_state.set(GameState::Playing);
                    }
                    MenuButtonAction::Pause => {
                        // Only meaningful while playing; ignore taps that leak
                        // through in other states.
                        if *state.get() == GameState::Playing {
                            next_state.set(GameState::Paused);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                transform.scale = Vec2::splat(HOVER_SCALE);
            }
            Interaction::None => {
                transform.scale = Vec2::splat(DEFAULT_SCALE);
            }
        }
    }
}
