use crate::hexgrid::PointyHexGrid;
use bevy::prelude::*;

pub struct OnTryOpenTile {
    target: PointyHexGrid,
}

pub struct OnTryFlagTile {
    target: PointyHexGrid,
}

pub enum OnMoveTile {
    Open { target: PointyHexGrid },
    Flag { target: PointyHexGrid },
}

pub struct OnUndoMoveTile;

pub struct OnGameOver;
pub struct OnGameClear;

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
            .add_event::<OnQuitGame>();
    }
}
