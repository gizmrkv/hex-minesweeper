use crate::events::*;
use crate::hexgrid;
use crate::model::GameBoard;
use crate::scene::GameScene;
use crate::view::Tilemap;
use crate::{Config, CursorWorldPosition};
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_try_move_tile_system)
            .add_system(check_retry_system)
            .add_system(check_try_undo_system);
    }
}

fn check_try_move_tile_system(
    mut on_try_open_tile_writer: EventWriter<OnTryOpenTile>,
    mut on_try_flag_tile_writer: EventWriter<OnTryFlagTile>,
    cursor_world_position: Res<CursorWorldPosition>,
    game_board: Res<GameBoard>,
    tilemap_query: Query<&Transform, With<Tilemap>>,
    buttons: Res<Input<MouseButton>>,
    config: Res<Config>,
    game_scene: Res<GameScene>,
) {
    let tilemap_transform = tilemap_query.single();
    let grid = hexgrid::cartesian_point_to_nearest_pointy_hex_grid(
        (cursor_world_position.position - tilemap_transform.translation.truncate())
            / config.tile_size,
    );

    if let GameScene::InGame = *game_scene {
        if !game_board.is_out_of_bound(grid) && buttons.just_released(MouseButton::Left) {
            on_try_open_tile_writer.send(OnTryOpenTile { target: grid });
        }
        if !game_board.is_out_of_bound(grid) && buttons.just_released(MouseButton::Right) {
            on_try_flag_tile_writer.send(OnTryFlagTile { target: grid });
        }
    }
}

fn check_retry_system(
    mut game_scene: ResMut<GameScene>,
    buttons: Res<Input<MouseButton>>,
    mut writer: EventWriter<OnRetry>,
) {
    match *game_scene {
        GameScene::Over | GameScene::Clear => {
            if buttons.just_released(MouseButton::Left) || buttons.just_released(MouseButton::Right)
            {
                *game_scene = GameScene::InGame;
                writer.send(OnRetry);
            }
        }
        _ => {}
    }
}

fn check_try_undo_system(
    game_scene: Res<GameScene>,
    mut writer: EventWriter<OnTryUndo>,
    keys: Res<Input<KeyCode>>,
) {
    if let GameScene::InGame = *game_scene {
        if keys.pressed(KeyCode::LControl) && keys.just_released(KeyCode::Z) {
            writer.send(OnTryUndo);
        }
    }
}
