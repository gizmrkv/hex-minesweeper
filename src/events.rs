use crate::hexgrid::PointyHexGrid;
use bevy::prelude::*;

#[derive(Debug)]
pub struct OnTryOpenTile {
    pub target: PointyHexGrid,
}

#[derive(Debug)]
pub struct OnTryFlagTile {
    pub target: PointyHexGrid,
}

#[derive(Debug)]
pub enum OnMoveTile {
    Open { target: PointyHexGrid },
    Flag { target: PointyHexGrid },
}

#[derive(Debug)]
pub struct OnUndoMoveTile;

#[derive(Debug)]
pub struct OnGameOver;
#[derive(Debug)]
pub struct OnGameClear;

#[derive(Debug)]
pub struct OnQuitGame;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnTryOpenTile>()
            .add_event::<OnTryFlagTile>()
            .add_event::<OnMoveTile>()
            .add_event::<OnUndoMoveTile>()
            .add_event::<OnGameOver>()
            .add_event::<OnGameClear>()
            .add_event::<OnQuitGame>()
            .add_system(info_on_try_open_tile_system)
            .add_system(info_on_move_tile_system)
            .add_system(info_on_try_flag_tile_system)
            .add_system(info_on_game_over_system);
    }
}

fn info_on_try_open_tile_system(mut reader: EventReader<OnTryOpenTile>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}

fn info_on_move_tile_system(mut reader: EventReader<OnMoveTile>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}

fn info_on_try_flag_tile_system(mut reader: EventReader<OnTryFlagTile>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}

fn info_on_game_over_system(mut reader: EventReader<OnGameOver>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}
