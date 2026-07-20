use bevy::prelude::*;

use crate::game::food::components::Food;
use crate::game::snake::components::{Snake, SnakeSegment};
use crate::game::snake::systems::spawn::spawn_new_snake;
use crate::game::ui::resources::Score;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_systems(Update, toggle_pause);
}

/// Despawns any leftover snake/food entities, resets the score, spawns a fresh
/// snake and transitions to `Playing`. Used by the menu's Start/Restart buttons.
pub fn start_new_game(
    commands: &mut Commands,
    snakes: &Query<Entity, With<Snake>>,
    segments: &Query<Entity, With<SnakeSegment>>,
    food: &Query<Entity, With<Food>>,
    score: &mut Score,
    next_state: &mut NextState<GameState>,
) {
    for entity in snakes.iter() {
        commands.entity(entity).despawn();
    }
    for entity in segments.iter() {
        commands.entity(entity).despawn();
    }
    for entity in food.iter() {
        commands.entity(entity).despawn();
    }
    score.0 = 0;

    spawn_new_snake(commands);

    next_state.set(GameState::Playing);
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    match state.get() {
        GameState::Playing => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::Playing),
        _ => {}
    }
}
