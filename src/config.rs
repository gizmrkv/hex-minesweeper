use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

use crate::AppState;

/// Plugin to allow access to app config.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Config>()
            .init_asset_loader::<ConfigLoader>()
            .add_startup_system(load_config)
            .add_system_set(
                SystemSet::on_update(AppState::LoadingConfig).with_system(enter_setup_state),
            )
            .add_system_set(SystemSet::on_exit(AppState::LoadingConfig).with_system(info_config));
    }
}

/// Load a config and spawn its handle.
fn load_config(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<Config> = asset_server.load("config/config.ron");
    commands.spawn(handle);
}

/// Enter Title from Setup.
fn enter_setup_state(mut app_state: ResMut<State<AppState>>, mut count: Local<usize>) {
    // Waiting to be able to get a config.
    if *count >= 1 {
        if let Ok(_) = app_state.set(AppState::Setup) {
            *count = 0;
        } else {
            error!("failed to transition app state.");
        }
    }
    *count += 1;
}

/// Inform config.
fn info_config(config_query: Query<&Handle<Config>>, config_assets: ResMut<Assets<Config>>) {
    let config_handle = config_query.single();
    if let Some(config) = config_assets.get(config_handle) {
        info!("{:#?}", config);
    } else {
        info!("No config.");
    }
}

/// app config.
#[derive(Debug, Deserialize, TypeUuid, Default, Clone, Copy)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct Config {
    pub background_color: Color,
}

/// app config loader.
#[derive(Default)]
struct ConfigLoader;

impl AssetLoader for ConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let config = ron::de::from_bytes::<Config>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(config));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
