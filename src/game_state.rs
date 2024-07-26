use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LevelSelection};

use crate::consts::DAYS_PER_SEASION;

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
            .init_resource::<TimeState>()
            .insert_resource(LevelSelection::index(0));
    }
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum TimeAxis {
    #[default]
    Horizontal,
    Vertical,
    None,
}

impl std::fmt::Display for TimeAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

#[derive(Resource, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TimeState {
    pub time_axis: TimeAxis,
    pub time_step_delta: i32,
    pub time: i32,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            time_axis: TimeAxis::Horizontal,
            time_step_delta: 5,
            time: 0,
        }
    }
}

#[derive(Default, Debug)]
pub enum Seasion {
    #[default]
    Spring,
    Summer,
    Autum,
    Winter,
}

impl std::fmt::Display for Seasion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl TimeState {
    pub fn current_hour(&self) -> i32 {
        self.time % 24
    }

    pub fn current_seasion(&self) -> Seasion {
        let day = self.time / 24;
        let seasion_int = (self.time / (DAYS_PER_SEASION * 24)) % 4;

        match seasion_int {
            0 => Seasion::Spring,
            1 => Seasion::Summer,
            2 => Seasion::Autum,
            3 => Seasion::Winter,
            _ => unreachable!("This should never happen"),
        }
    }
}
