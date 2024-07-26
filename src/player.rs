use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    game_state::{GameState, TimeAxis, TimeState},
    goal::Goal,
    orbs::{AxisSwitch, DirectionSwitch, SpeedUp},
    walls::LevelWalls,
};

#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    animation_timer: PlayerAnimationTimer,
}

pub struct PlayerPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerAnimationTimer(Timer);

impl Default for PlayerAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(
                Update,
                (
                    move_player_from_input,
                    animate_player,
                    check_switch_orb_hit,
                    check_direction_orb_hit,
                    check_slow_down_orb_hit,
                    check_speed_up_orb_hit,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

/// Basic player movement system
fn move_player_from_input(
    mut players: Query<(&mut GridCoords, &mut Sprite), With<Player>>,
    mut time_state: ResMut<TimeState>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let (movement_direction, facing, axis, sense) = if input.just_pressed(KeyCode::KeyW) {
        (GridCoords::new(0, 1), Facing::Up, TimeAxis::Vertical, 1)
    } else if input.just_pressed(KeyCode::KeyA) {
        (
            GridCoords::new(-1, 0),
            Facing::Left,
            TimeAxis::Horizontal,
            -1,
        )
    } else if input.just_pressed(KeyCode::KeyS) {
        (GridCoords::new(0, -1), Facing::Down, TimeAxis::Vertical, -1)
    } else if input.just_pressed(KeyCode::KeyD) {
        (
            GridCoords::new(1, 0),
            Facing::Right,
            TimeAxis::Horizontal,
            1,
        )
    } else {
        return;
    };

    /// TODO break this out into another system
    if (time_state.time_axis == axis) {
        time_state.time = time_state.time + sense * time_state.time_step_delta
    }

    for (mut player_grid_coords, mut sprite) in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
        match facing {
            Facing::Left => sprite.flip_x = true,
            Facing::Right => sprite.flip_x = false,
            _ => {}
        }
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut PlayerAnimationTimer), With<Player>>,
) {
    for (mut atlas, mut timer) in &mut query {
        timer.tick(time.delta());
        if (timer.just_finished()) {
            atlas.index = atlas.index + 1;
            if atlas.index > 143 + 5 {
                atlas.index = 143
            }
        }
    }
}

// Switch orb!
fn check_switch_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut orbs: Query<&GridCoords, With<AxisSwitch>>,
) {
    if players
        .iter()
        .zip(orbs.iter())
        .any(|(player_grid_coords, orb_grid_coords)| player_grid_coords == orb_grid_coords)
    {
        let new_axis = match time_state.time_axis {
            TimeAxis::Horizontal => TimeAxis::Vertical,
            TimeAxis::Vertical => TimeAxis::Horizontal,
            TimeAxis::None => TimeAxis::None,
        };
        time_state.time_axis = new_axis;
    }
}

// Direction orb!
fn check_direction_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut orbs: Query<&GridCoords, With<DirectionSwitch>>,
) {
    if players
        .iter()
        .zip(orbs.iter())
        .any(|(player_grid_coords, orb_grid_coords)| player_grid_coords == orb_grid_coords)
    {
        time_state.time_step_delta = -time_state.time_step_delta;
    }
}

// Speed up orb!
fn check_speed_up_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut orbs: Query<&GridCoords, With<SpeedUp>>,
) {
    if players
        .iter()
        .zip(orbs.iter())
        .any(|(player_grid_coords, orb_grid_coords)| player_grid_coords == orb_grid_coords)
    {
        time_state.time_step_delta = time_state.time_step_delta + 1;
    }
}

// Slow down orb!
fn check_slow_down_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut orbs: Query<&GridCoords, With<SpeedUp>>,
) {
    if players
        .iter()
        .zip(orbs.iter())
        .any(|(player_grid_coords, orb_grid_coords)| player_grid_coords == orb_grid_coords)
    {
        time_state.time_step_delta = time_state.time_step_delta - 1;
    }
}

/// Did the player reach the goal?
fn check_goal_acheived(
    level_selection: ResMut<LevelSelection>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    goals: Query<&GridCoords, With<Goal>>,
) {
    if players
        .iter()
        .zip(goals.iter())
        .any(|(player_grid_coords, goal_grid_coords)| player_grid_coords == goal_grid_coords)
    {
        println!("GOAL ACHEIVED");
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        indices.level += 1;
    }
}
