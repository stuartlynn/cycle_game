use bevy::{a11y::accesskit::Vec2, prelude::*, render::render_resource::AsBindGroupShaderType};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    consts,
    game_state::{GameState, TimeAxis, TimeState},
    goal::Goal,
    orbs::{AxisSwitch, DirectionSwitch, Orb, SlowDown, SpeedUp},
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
                (move_player_from_input, animate_player, check_goal_acheived)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    check_switch_orb_hit,
                    check_direction_orb_hit,
                    check_slow_down_orb_hit,
                    check_speed_up_orb_hit,
                    check_in_no_orb,
                    check_in_orb,
                )
                    .chain()
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
    mut players: Query<(&mut Transform, &mut GridCoords, &mut Sprite), With<Player>>,
    mut time_state: ResMut<TimeState>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let (movement, facing, axis, sense) = if input.pressed(KeyCode::KeyW) {
        ((0.0, 1.0), Facing::Up, TimeAxis::Vertical, 1)
    } else if input.pressed(KeyCode::KeyA) {
        ((-1.0, 0.0), Facing::Left, TimeAxis::Horizontal, -1)
    } else if input.pressed(KeyCode::KeyS) {
        ((0.0, -1.0), Facing::Down, TimeAxis::Vertical, -1)
    } else if input.pressed(KeyCode::KeyD) {
        ((1.0, 0.0), Facing::Right, TimeAxis::Horizontal, 1)
    } else {
        return;
    };

    // TODO break this out into another system

    for (mut transform, mut player_grid_coords, mut sprite) in players.iter_mut() {
        let translation = Vec2::from(movement) * consts::MOVEMENT_SPEED;
        let new_transform = transform.with_translation(
            transform.translation
                + Vec3 {
                    x: translation.x as f32,
                    y: translation.y as f32,
                    z: 0.0,
                },
        );

        let new_grid_coords = bevy_ecs_ldtk::utils::translation_to_grid_coords(
            new_transform.translation.xy(),
            IVec2 {
                x: consts::GRID_SIZE,
                y: consts::GRID_SIZE,
            },
        );

        if !level_walls.in_wall(&new_grid_coords) {
            *player_grid_coords = new_grid_coords;
            *transform = new_transform;
            if time_state.time_axis == axis {
                time_state.time = time_state.time + sense * time_state.time_step_delta
            }
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
    players: Query<&GridCoords, (With<Player>, Without<PlayerInOrb>, Changed<GridCoords>)>,
    orbs: Query<&GridCoords, With<AxisSwitch>>,
) {
    for player in &players {
        for orb in &orbs {
            if player == orb {
                let new_axis = match time_state.time_axis {
                    TimeAxis::Horizontal => TimeAxis::Vertical,
                    TimeAxis::Vertical => TimeAxis::Horizontal,
                    TimeAxis::None => TimeAxis::None,
                };
                time_state.time_axis = new_axis;
                return;
            }
        }
    }
}

#[derive(Component)]
pub struct PlayerInOrb;

pub fn check_in_orb(
    mut commands: Commands,
    players: Query<
        (Entity, &GridCoords),
        (With<Player>, Without<PlayerInOrb>, Changed<GridCoords>),
    >,
    orbs: Query<&GridCoords, With<Orb>>,
) {
    for (player, player_coords) in &players {
        for orb_coords in &orbs {
            if player_coords == orb_coords {
                commands.entity(player).insert(PlayerInOrb);
                return;
            }
        }
    }
}

pub fn check_in_no_orb(
    mut commands: Commands,
    players: Query<(Entity, &GridCoords), (With<Player>, With<PlayerInOrb>, Changed<GridCoords>)>,
    orbs: Query<&GridCoords, With<Orb>>,
) {
    for (player, player_coords) in &players {
        for orb_coords in &orbs {
            if player_coords == orb_coords {
                return;
            }
        }
        commands.entity(player).remove::<PlayerInOrb>();
    }
}
// Direction orb!
fn check_direction_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Without<PlayerInOrb>, Changed<GridCoords>)>,
    orbs: Query<&GridCoords, With<DirectionSwitch>>,
) {
    for player in &players {
        for orb in &orbs {
            if player == orb {
                time_state.time_step_delta = -time_state.time_step_delta;
                return;
            }
        }
    }
}

// Speed up orb!
fn check_speed_up_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Without<PlayerInOrb>, Changed<GridCoords>)>,
    orbs: Query<&GridCoords, With<SpeedUp>>,
) {
    for player in &players {
        for orb in &orbs {
            if player == orb {
                time_state.time_step_delta = time_state.time_step_delta + 1;
                return;
            }
        }
    }
}

// Slow down orb!
fn check_slow_down_orb_hit(
    mut time_state: ResMut<TimeState>,
    players: Query<&GridCoords, (With<Player>, Without<PlayerInOrb>, Changed<GridCoords>)>,
    orbs: Query<&GridCoords, With<SlowDown>>,
) {
    for player in &players {
        for orb in &orbs {
            if player == orb {
                time_state.time_step_delta = time_state.time_step_delta - 1;
                return;
            }
        }
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
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        indices.level += 1;
    }
}
