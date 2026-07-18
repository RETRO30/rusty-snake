use bevy::prelude::*;

use super::resources::Score;
use super::systems::spawn::spawn_score_ui;
use super::systems::update::update_score_text;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>();
    app.add_systems(Startup, spawn_score_ui);
    app.add_systems(Update, update_score_text);
}
