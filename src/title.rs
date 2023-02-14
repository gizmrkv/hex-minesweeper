use crate::AppState;
use bevy::prelude::*;

/// Plugin that controls the title state.
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Title).with_system(spawn_title_image));
    }
}

/// Spawn title image.
fn spawn_title_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("image/title.png");
    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        ..Default::default()
    });
}
