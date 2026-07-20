use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub u32);

/// Font used for in-game UI text. Bevy's embedded default font only covers
/// basic Latin, so glyphs like the arrow keys (↑↓←→) render blank with it;
/// DejaVu Sans has full arrow (and Cyrillic) coverage.
#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

impl FromWorld for UiFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        UiFont(asset_server.load("fonts/DejaVuSans.ttf"))
    }
}
