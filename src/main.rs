use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
pub use std::io::*;

mod controller;
mod events;
mod hexgrid;
mod model;
mod read_macro;
mod scene;
mod view;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(Config {
            tile_size: 50.0,
            tile_text_size: 50.0,
            tile_gap_scale: 0.95,
            tile_layer: 0.1,
            tile_edge_layer: 0.0,
            tile_text_layer: 0.2,
            game_over_background_layer: 1.0,
            game_over_text_layer: 1.1,
            game_clear_background_layer: 1.0,
            game_clear_text_layer: 1.1,
            mines_hint_text_layer: 0.9,
            tile_color: Color::rgb(0.1, 0.1, 0.1),
            tile_edge_color: Color::rgb(0.8, 0.8, 0.8),
            tile_selected_color: Color::rgb(0.4, 0.4, 0.4),
            tile_text_hint_color: Color::rgb(0.9, 0.9, 0.9),
            tile_text_flag_color: Color::rgb(0.8, 0.8, 0.0),
            tile_text_mine_color: Color::rgb(0.8, 0.0, 0.0),
            game_over_background_color: Color::rgba(0.0, 0.0, 0.0, 0.9),
            game_over_text_color: Color::rgb(0.9, 0.9, 0.9),
            game_over_text_position: Vec2 { x: 0.0, y: 20.0 },
            game_over_text_size: 100.0,
            game_over_wrong_open_text: "You Exploded".to_string(),
            game_over_wrong_flag_text: "Flag Safe Tile".to_string(),
            game_over_text_below_color: Color::rgb(0.9, 0.9, 0.9),
            game_over_text_below_position: Vec2 { x: 0.0, y: -60.0 },
            game_over_text_below_size: 40.0,
            game_over_text_below: "click anywhere to retry".to_string(),
            game_clear_background_color: Color::rgba(0.0, 0.0, 0.0, 0.9),
            game_clear_text_color: Color::rgb(0.9, 0.9, 0.9),
            game_clear_text_position: Vec2 { x: 0.0, y: 20.0 },
            game_clear_text_size: 100.0,
            game_clear_text: "Stage Clear!".to_string(),
            game_clear_text_below_color: Color::rgb(0.9, 0.9, 0.9),
            game_clear_text_below_position: Vec2 { x: 0.0, y: -60.0 },
            game_clear_text_below_size: 40.0,
            game_clear_text_below: "click anywhere to retry".to_string(),
            tile_text_font_path: "fonts/FiraSans-Bold.ttf".to_string(),
            game_over_text_font_path: "fonts/FiraSans-Bold.ttf".to_string(),
            game_clear_text_font_path: "fonts/FiraSans-Bold.ttf".to_string(),
            sound_click_path: "sound/click.ogg".to_string(),
            sound_game_over_path: "sound/bomb.ogg".to_string(),
            sound_game_clear_path: "sound/cym.ogg".to_string(),
            mines_hint_text_position: Vec2 {
                x: -620.0,
                y: 350.0,
            },
            mines_hint_text: "Mines: {} (Remaining: {})".to_string(),
            mines_hint_text_color: Color::rgb(0.9, 0.9, 0.9),
            mines_hint_text_font_path: "fonts/FiraSans-Bold.ttf".to_string(),
            mines_hint_text_size: 40.0,
        })
        .insert_resource(CursorWorldPosition {
            position: Vec2::ZERO,
        })
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, update_cursor_world_position_system)
        .add_plugin(model::ModelPlugin)
        .add_plugin(view::ViewPlugin)
        .add_plugin(controller::ControllerPlugin)
        .add_plugin(events::EventsPlugin)
        .add_plugin(scene::ScenePlugin)
        .run();
}

#[derive(Resource)]
pub struct Config {
    pub tile_size: f32,
    pub tile_gap_scale: f32,

    pub tile_layer: f32,
    pub tile_edge_layer: f32,
    pub tile_text_layer: f32,
    pub game_over_background_layer: f32,
    pub game_over_text_layer: f32,
    pub game_clear_background_layer: f32,
    pub game_clear_text_layer: f32,
    pub mines_hint_text_layer: f32,

    pub tile_color: Color,
    pub tile_edge_color: Color,
    pub tile_selected_color: Color,
    pub tile_text_hint_color: Color,
    pub tile_text_flag_color: Color,
    pub tile_text_mine_color: Color,
    pub game_over_background_color: Color,
    pub game_over_text_color: Color,
    pub game_clear_background_color: Color,
    pub game_clear_text_color: Color,

    pub tile_text_font_path: String,
    pub tile_text_size: f32,
    pub game_over_text_font_path: String,
    pub game_over_text_size: f32,
    pub game_over_text_position: Vec2,
    pub game_over_wrong_open_text: String,
    pub game_over_wrong_flag_text: String,

    pub game_over_text_below_color: Color,
    pub game_over_text_below_position: Vec2,
    pub game_over_text_below_size: f32,
    pub game_over_text_below: String,

    pub game_clear_text_font_path: String,
    pub game_clear_text_size: f32,
    pub game_clear_text_position: Vec2,
    pub game_clear_text: String,

    pub game_clear_text_below_color: Color,
    pub game_clear_text_below_position: Vec2,
    pub game_clear_text_below_size: f32,
    pub game_clear_text_below: String,

    pub sound_click_path: String,
    pub sound_game_over_path: String,
    pub sound_game_clear_path: String,

    pub mines_hint_text_color: Color,
    pub mines_hint_text_position: Vec2,
    pub mines_hint_text_size: f32,
    pub mines_hint_text_font_path: String,
    pub mines_hint_text: String,
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());

    if let Some(window) = windows.get_primary_mut() {
        window.set_resizable(false);
    }
}

#[derive(Resource)]
struct CursorWorldPosition {
    position: Vec2,
}

fn update_cursor_world_position_system(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut cursor_world_position: ResMut<CursorWorldPosition>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor_world_position.position = world_position.truncate();
    }
}
