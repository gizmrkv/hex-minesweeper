use crate::hexgrid::PointyHexGrid;

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

pub struct OnGameOver;
pub struct OnGameClear;

pub struct OnQuitGame;
