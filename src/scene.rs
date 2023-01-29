use crate::events::*;
use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameScene::InGame)
            .add_system(check_game_over_system);
    }
}

#[derive(Resource)]
pub enum GameScene {
    Over,
    Clear,
    InGame,
}

fn check_game_over_system(mut reader: EventReader<OnGameOver>, mut scene: ResMut<GameScene>) {
    for event in reader.iter() {
        *scene = GameScene::Over;
    }
}
