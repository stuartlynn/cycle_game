// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod consts;
mod goal;
mod player;
mod walls;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use goal::GoalPlugin;
use player::PlayerPlugin;
use walls::WallPlugin;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 900.0 / 4.0;
    camera.transform.translation.y += 500.0 / 4.0;

    // camera.transform.translation.x += 1280.0 / 4.0;
    // camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Cycle Game"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WallPlugin)
        .add_plugins(GoalPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, translate_grid_coords_entities)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::splat(consts::GRID_SIZE),
        )
        .extend(transform.translation.z);
    }
}
