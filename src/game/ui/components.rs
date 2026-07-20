use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct PauseRoot;

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuButtonAction {
    Start,
    Resume,
    Restart,
    /// The in-play pause button in the score bar (mainly for touch devices,
    /// which have no Esc key).
    Pause,
}
