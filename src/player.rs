use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{goal::Goal, walls::LevelWalls};

#[derive(Component, Default)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (move_player_from_input, test_system));
    }
}

fn test_system(mut players: Query<(&GridCoords, &mut Sprite, &TextureAtlas), With<Player>>) {
    for (coords, mut sprite, texture_atlas) in &mut players {
        println!("Got sprite {:#?}", sprite);
        println!("Got Atlas {:#?}", sprite);
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
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let (movement_direction, facing) = if input.just_pressed(KeyCode::KeyW) {
        (GridCoords::new(0, 1), Facing::Up)
    } else if input.just_pressed(KeyCode::KeyA) {
        (GridCoords::new(-1, 0), Facing::Left)
    } else if input.just_pressed(KeyCode::KeyS) {
        (GridCoords::new(0, -1), Facing::Down)
    } else if input.just_pressed(KeyCode::KeyD) {
        (GridCoords::new(1, 0), Facing::Right)
    } else {
        return;
    };

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
