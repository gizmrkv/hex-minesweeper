use crate::events::*;
use crate::hexgrid;
use crate::model::GameBoard;
use crate::view::Tilemap;
use crate::{Config, CursorWorldPosition};
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_try_open_tile_system);
    }
}

fn check_try_open_tile_system(
    mut writer: EventWriter<OnTryOpenTile>,
    cursor_world_position: Res<CursorWorldPosition>,
    game_board: Res<GameBoard>,
    tilemap_query: Query<&Transform, With<Tilemap>>,
    buttons: Res<Input<MouseButton>>,
    config: Res<Config>,
) {
    let tilemap_transform = tilemap_query.single();
    let grid = hexgrid::cartesian_point_to_nearest_pointy_hex_grid(
        (cursor_world_position.position - tilemap_transform.translation.truncate())
            / config.tile_size,
    );

    if !game_board.is_out_of_bound(grid) && buttons.just_released(MouseButton::Left) {
        writer.send(OnTryOpenTile { target: grid });
    }
}
