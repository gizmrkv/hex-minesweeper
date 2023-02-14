use bevy::log::LogPlugin;
use bevy::prelude::*;

mod config;
mod cursor2d;
mod hexagonal_coordinate;
mod hexagonal_cursor;

/// App state.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum AppState {
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
        .add_state(AppState::Setup)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(enter_title))
        .add_system_set(SystemSet::on_exit(AppState::Setup).with_system(info_config))
        .run();
}

/// Spawn camera2d.
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Show config.
fn info_config(
    config_query: Query<&Handle<config::Config>>,
    config_assets: ResMut<Assets<config::Config>>,
) {
    let config_handle = config_query.single();
    if let Some(config) = config_assets.get(config_handle) {
        info!("{:#?}", config);
    } else {
        info!("No config.");
    }
}

/// Enter Title from Setup.
fn enter_title(mut app_state: ResMut<State<AppState>>, mut count: Local<usize>) {
    // Waiting to be able to get a config.
    if *count >= 1 {
        if let Ok(_) = app_state.set(AppState::Title) {
        } else {
            error!("failed to transition app state.");
        }
    }
    *count += 1;
}
