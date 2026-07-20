use bevy::prelude::*;

use crate::game::ui::components::ScoreText;
use crate::game::ui::resources::Score;

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut query {
        text.0 = format!("Score: {}", score.0);
    }
}
