use bevy::prelude::*;

mod controller;
mod events;
mod hexgrid;
mod model;
mod view;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_startup_system(setup)
        .insert_resource(view::ViewConfig {
            tile_size: 50.0,
            tile_gap_scale: 0.95,
            tile_layer: 0.0,
            tile_color: Color::rgb(0.1, 0.1, 0.1),
            tile_edge_color: Color::rgb(0.8, 0.8, 0.8),
            tile_text_font_path: "fonts/FiraSans-Bold.ttf".to_string(),
            tile_text_size: 50.0,
            tile_text_color: Color::rgb(0.9, 0.9, 0.9),
            tile_text_layer: 1.0,
        })
        .add_plugin(model::ModelPlugin)
        .add_plugin(view::ViewPlugin)
        .add_plugin(controller::ControllerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
