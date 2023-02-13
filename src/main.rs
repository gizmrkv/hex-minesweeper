use bevy::log::LogPlugin;
use bevy::prelude::*;

mod cursor2d;
mod hexagonal_coordinate;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Title,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_plugin(cursor2d::Cursor2dPlugin)
        .add_state(GameState::Title)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
