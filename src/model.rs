use crate::events;
use crate::hexgrid;
use crate::hexgrid::PointyHexGrid;
use crate::read_to_end;
use bevy::prelude::*;
use std::fs;
use std::io::*;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(game_board) = GameBoard::load(1) {
            app.insert_resource(game_board)
                .add_system(on_try_open_tile_system)
                .add_system(on_try_flag_tile_system);
        } else {
            debug_assert!(true, "failed to load game board");
        }
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

#[derive(Resource, Default, Debug)]
pub struct GameBoard {
    tiles_per_side: usize,
    board: Vec<TileState>,
}

impl GameBoard {
    pub fn new(tiles_per_side: usize) -> Self {
        Self {
            tiles_per_side,
            board: vec![default(); (2 * tiles_per_side - 1) * (2 * tiles_per_side - 1)],
        }
    }

    pub fn load(id: usize) -> Result<Self> {
        use crate::*;
        let stdin = fs::File::open(format!("assets/boards/{}.txt", id))?;
        let mut reader = BufReader::new(stdin);
        read_to_end!(
            reader,
            tiles_per_side: usize,
            board_text: [chars; 2 * tiles_per_side - 1]
        );

        let mut board = Self {
            tiles_per_side,
            board: vec![default(); (2 * tiles_per_side - 1) * (2 * tiles_per_side - 1)],
        };

        for x in 0..(2 * tiles_per_side - 1) {
            for y in 0..(2 * tiles_per_side - 1) {
                if let Some(tile_state) = board.get_mut(PointyHexGrid {
                    x: x as i32,
                    y: y as i32,
                }) {
                    *tile_state = match board_text[y][x] {
                        '.' => TileState {
                            is_open: false,
                            is_flag: false,
                            is_mine: false,
                        },
                        'O' => TileState {
                            is_open: true,
                            is_flag: false,
                            is_mine: false,
                        },
                        'M' => TileState {
                            is_open: false,
                            is_flag: false,
                            is_mine: true,
                        },
                        _ => {
                            panic!("board text error! : {}", board_text[y][x])
                        }
                    }
                }
            }
        }

        Ok(board)
    }

    pub fn tiles_per_side(&self) -> usize {
        self.tiles_per_side
    }

    pub fn get(&self, grid: hexgrid::PointyHexGrid) -> Option<&TileState> {
        if self.is_out_of_bound(grid) {
            None
        } else {
            self.board
                .get((grid.y * (2 * self.tiles_per_side as i32 - 1) + grid.x) as usize)
        }
    }

    pub fn get_mut(&mut self, grid: hexgrid::PointyHexGrid) -> Option<&mut TileState> {
        if self.is_out_of_bound(grid) {
            None
        } else {
            self.board
                .get_mut((grid.y * (2 * self.tiles_per_side as i32 - 1) + grid.x) as usize)
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

    pub fn count_open_tile(&self) -> usize {
        self.board
            .iter()
            .filter(|tile_state| tile_state.is_open)
            .count()
    }

    pub fn count_mines(&self) -> usize {
        self.board
            .iter()
            .filter(|tile_state| tile_state.is_mine)
            .count()
    }

    pub fn count_flagged_mines(&self) -> usize {
        self.board
            .iter()
            .filter(|tile_state| tile_state.is_mine && tile_state.is_flag)
            .count()
    }

    pub fn count_remaining_mines(&self) -> usize {
        self.count_mines() - self.count_flagged_mines()
    }
}

fn on_try_open_tile_system(
    mut game_board: ResMut<GameBoard>,
    mut reader: EventReader<events::OnTryOpenTile>,
    mut writer: EventWriter<events::OnMoveTile>,
    mut game_over_writer: EventWriter<events::OnGameOver>,
) {
    for event in reader.iter() {
        if let Some(tile_state) = game_board.get_mut(event.target) {
            if !tile_state.is_open && !tile_state.is_flag {
                tile_state.is_open = true;
                writer.send(events::OnMoveTile::Open {
                    target: event.target,
                });
            }
            if tile_state.is_mine {
                game_over_writer.send(events::OnGameOver);
            }
        }
    }
}

fn on_try_flag_tile_system(
    mut game_board: ResMut<GameBoard>,
    mut reader: EventReader<events::OnTryFlagTile>,
    mut writer: EventWriter<events::OnMoveTile>,
) {
    for event in reader.iter() {
        if let Some(tile_state) = game_board.get_mut(event.target) {
            if !tile_state.is_open && !tile_state.is_flag {
                tile_state.is_flag = true;
                writer.send(events::OnMoveTile::Flag {
                    target: event.target,
                });
            }
        }
    }
}
