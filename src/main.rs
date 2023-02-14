use bevy::log::LogPlugin;
use bevy::prelude::*;

mod config;
mod cursor2d;
mod hexagonal_coordinate;
mod hexagonal_cursor;
mod hexagonal_table;
mod title;

/// App state.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum AppState {
    LoadingConfig,
    Setup,
    Title,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugin(cursor2d::Cursor2dPlugin)
        .add_plugin(config::ConfigPlugin)
        .add_plugin(hexagonal_cursor::HexagonalCursorPlugin)
        .add_state(AppState::LoadingConfig)
        .add_startup_system(spawn_camera)
        .run();
}

/// Spawn camera2d.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
