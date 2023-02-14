use bevy::log::LogPlugin;
use bevy::prelude::*;

mod config;
mod cursor2d;
mod hexagonal_coordinate;
mod hexagonal_cursor;
mod hexagonal_table;
mod title;

use config::*;
use cursor2d::*;
use hexagonal_coordinate::*;
use hexagonal_cursor::*;
use hexagonal_table::*;
use title::*;

/// App state.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum AppState {
    Title,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugin(Cursor2dPlugin)
        .add_plugin(ConfigPlugin)
        .add_plugin(HexagonalCursorPlugin)
        .add_plugin(TitlePlugin)
        .add_state(AppState::Title)
        .add_startup_system(spawn_camera)
        .run();
}

/// Spawn camera2d.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
