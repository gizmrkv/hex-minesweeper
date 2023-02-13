use bevy::prelude::*;

mod hexagonal_coordinate;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Title,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Title)
        .run();
}
