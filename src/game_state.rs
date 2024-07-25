use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LevelSelection};

#[derive(Resource, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    WelcomeScreen,
    Playing,
    Dead,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(LevelSelection::index(0));
    }
}
