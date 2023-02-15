use crate::rule::*;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
use std::collections::HashMap;

/// Plugin to allow access to app config.
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Config>()
            .init_asset_loader::<ConfigLoader>()
            .add_startup_system(load_config)
            .add_system(info_config);
    }
}

/// Load a config and spawn its handle.
fn load_config(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<Config> = asset_server.load("config/config.ron");
    commands.spawn(handle);
}

/// Inform config.
fn info_config(
    config_assets: ResMut<Assets<Config>>,
    mut asset_event: EventReader<AssetEvent<Config>>,
) {
    for event in asset_event.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(config) = config_assets.get(handle) {
                    info!("\n{:#?}", config);
                }
            }
            _ => {}
        }
    }
}

/// app config.
#[derive(Debug, Deserialize, TypeUuid, Default, Clone)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct Config {
    pub background_color: Color,
    pub menu_layout: HashMap<(i32, i32), StageRule>,
}

/// App config loader.
#[derive(Default)]
struct ConfigLoader;

impl AssetLoader for ConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = ron::de::from_bytes::<Config>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
