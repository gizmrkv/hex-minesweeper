use crate::events;
use crate::hexgrid;
use bevy::prelude::*;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameBoard::new(4))
            .add_system(on_try_open_tile_system)
            .add_system(on_try_flag_tile_system);
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TileState {
    is_open: bool,
    is_flag: bool,
    is_mine: bool,
}

impl TileState {
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn is_flag(&self) -> bool {
        self.is_flag
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }
}

#[derive(Resource)]
pub struct GameBoard {
    tiles_per_side: usize,
    board: Vec<TileState>,
}

impl GameBoard {
    pub fn new(side_size: usize) -> Self {
        let mut board = Self {
            tiles_per_side: side_size,
            board: vec![default(); (2 * side_size - 1) * (2 * side_size - 1)],
        };
        if let Some(tile) = board.get_mut(hexgrid::PointyHexGrid { x: 4, y: 4 }) {
            tile.is_open = true;
        }
        if let Some(tile) = board.get_mut(hexgrid::PointyHexGrid { x: 5, y: 4 }) {
            tile.is_mine = true;
        }
        board
    }

    pub fn tiles_per_side(&self) -> usize {
        self.tiles_per_side
    }

    pub fn get(&self, grid: hexgrid::PointyHexGrid) -> Option<&TileState> {
        if self.is_out_of_bound(grid) {
            None
        } else {
            self.board
                .get((grid.y * 2 * (self.tiles_per_side as i32 - 1) + grid.x) as usize)
        }
    }

    pub fn get_mut(&mut self, grid: hexgrid::PointyHexGrid) -> Option<&mut TileState> {
        if self.is_out_of_bound(grid) {
            None
        } else {
            self.board
                .get_mut((grid.y * 2 * (self.tiles_per_side as i32 - 1) + grid.x) as usize)
        }
    }

    pub fn is_out_of_bound(&self, grid: hexgrid::PointyHexGrid) -> bool {
        grid.x < 0
            || grid.x >= (2 * self.tiles_per_side - 1) as i32
            || grid.y < 0
            || grid.y >= (2 * self.tiles_per_side - 1) as i32
            || grid.x + grid.y < (self.tiles_per_side - 1) as i32
            || grid.x + grid.y > 3 * (self.tiles_per_side - 1) as i32
    }

    pub fn count_adjacent_mines(&self, grid: hexgrid::PointyHexGrid) -> Option<usize> {
        if self.is_out_of_bound(grid) {
            None
        } else {
            let mut count = 0;
            let diff = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
            for [dx, dy] in diff {
                if let Some(tile_state) = self.get(hexgrid::PointyHexGrid {
                    x: grid.x + dx,
                    y: grid.y + dy,
                }) {
                    if tile_state.is_mine {
                        count += 1;
                    }
                }
            }
            Some(count)
        }
    }

    pub fn try_open_tile(&mut self, grid: hexgrid::PointyHexGrid) -> bool {
        if let Some(tile_state) = self.get_mut(grid) {
            if tile_state.is_open {
                false
            } else {
                tile_state.is_open = true;
                true
            }
        } else {
            false
        }
    }

    pub fn try_flag_tile(&mut self, grid: hexgrid::PointyHexGrid) -> bool {
        if let Some(tile_state) = self.get_mut(grid) {
            if !tile_state.is_open && !tile_state.is_flag {
                tile_state.is_flag = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn on_try_open_tile_system(
    mut game_board: ResMut<GameBoard>,
    mut reader: EventReader<events::OnTryOpenTile>,
    mut writer: EventWriter<events::OnMoveTile>,
) {
    for event in reader.iter() {
        if game_board.try_open_tile(event.target) {
            writer.send(events::OnMoveTile::Open {
                target: event.target,
            });
        }
    }
}

fn on_try_flag_tile_system(
    mut game_board: ResMut<GameBoard>,
    mut reader: EventReader<events::OnTryFlagTile>,
    mut writer: EventWriter<events::OnMoveTile>,
) {
    for event in reader.iter() {
        if game_board.try_flag_tile(event.target) {
            writer.send(events::OnMoveTile::Flag {
                target: event.target,
            });
        }
    }
}
