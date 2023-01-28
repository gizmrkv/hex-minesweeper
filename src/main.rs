use bevy::prelude::*;

mod controller;
mod events;
mod hexgrid;
mod model;
mod view;

use std::fs;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(view::ViewConfig {
            tile_size: 30.0,
            tile_gap_scale: 0.95,
            tile_layer: 0.0,
            tile_color: Color::rgb(1.0, 0.5, 0.5),
            tile_text_font_path: "fonts/FiraSans-Bold.ttf".to_string(),
            tile_text_size: 30.0,
            tile_text_color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_plugin(model::ModelPlugin)
        .add_plugin(view::ViewPlugin)
        .add_plugin(controller::ControllerPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
