use crate::AppState;
use bevy::prelude::*;

/// Plugin that controls the title state.
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Setup).with_system(enter_title_state))
            .add_system_set(SystemSet::on_enter(AppState::Title).with_system(spawn_title_image));
    }
}

/// Enter Title from Setup.
fn enter_title_state(mut app_state: ResMut<State<AppState>>, mut count: Local<usize>) {
    // Waiting to be able to get a config.
    if *count >= 1 {
        if let Ok(_) = app_state.set(AppState::Title) {
            *count = 0;
        } else {
            error!("failed to transition app state.");
        }
    }
    *count += 1;
}

/// Spawn title image.
fn spawn_title_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("image/title.png");
    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        ..Default::default()
    });
}
