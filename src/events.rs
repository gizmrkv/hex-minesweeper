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

#[derive(Debug, Clone, Copy)]
pub enum OnMoveTile {
    Open { target: PointyHexGrid },
    Flag { target: PointyHexGrid },
}

#[derive(Debug, Clone, Copy)]
pub struct OnTryUndo;

#[derive(Debug, Clone, Copy)]
pub enum OnUndoTile {
    UnOpen { target: PointyHexGrid },
    UnFlag { target: PointyHexGrid },
}

#[derive(Debug)]
pub enum OnGameOver {
    Open { target: PointyHexGrid },
    Flag { target: PointyHexGrid },
}
#[derive(Debug)]
pub struct OnGameClear;

#[derive(Debug)]
pub struct OnQuitGame;

#[derive(Debug)]
pub struct OnRetry;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnTryOpenTile>()
            .add_event::<OnTryFlagTile>()
            .add_event::<OnMoveTile>()
            .add_event::<OnGameOver>()
            .add_event::<OnGameClear>()
            .add_event::<OnQuitGame>()
            .add_event::<OnRetry>()
            .add_event::<OnTryUndo>()
            .add_event::<OnUndoTile>()
            .add_system(info_on_try_open_tile_system)
            .add_system(info_on_move_tile_system)
            .add_system(info_on_try_flag_tile_system)
            .add_system(info_on_game_over_system)
            .add_system(info_on_game_clear_system)
            .add_system(info_on_retry_system)
            .add_system(info_on_try_undo_system)
            .add_system(info_on_undo_tile_system);
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

fn info_on_game_clear_system(mut reader: EventReader<OnGameClear>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}

fn info_on_retry_system(mut reader: EventReader<OnRetry>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}

fn info_on_try_undo_system(mut reader: EventReader<OnTryUndo>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}

fn info_on_undo_tile_system(mut reader: EventReader<OnUndoTile>) {
    for event in reader.iter() {
        info!("{:?}", event);
    }
}
