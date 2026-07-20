use bevy::prelude::*;

use crate::game::state::GameState;

use super::resources::{Score, UiFont};
use super::systems::buttons::handle_menu_buttons;
use super::systems::game_over::{despawn_game_over, spawn_game_over};
use super::systems::menu::{despawn_menu, spawn_menu};
use super::systems::pause::{despawn_pause, spawn_pause};
use super::systems::spawn::spawn_score_ui;
use super::systems::update::update_score_text;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>();
    app.init_resource::<UiFont>();
    app.add_systems(Startup, spawn_score_ui);
    app.add_systems(Update, (update_score_text, handle_menu_buttons));

    app.add_systems(OnEnter(GameState::Menu), spawn_menu);
    app.add_systems(OnExit(GameState::Menu), despawn_menu);

    app.add_systems(OnEnter(GameState::Paused), spawn_pause);
    app.add_systems(OnExit(GameState::Paused), despawn_pause);

    app.add_systems(OnEnter(GameState::GameOver), spawn_game_over);
    app.add_systems(OnExit(GameState::GameOver), despawn_game_over);
}
