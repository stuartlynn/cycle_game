// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

// use bevy::asset::AssetMetaCheck;
// use bevy::prelude::*;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(AssetPlugin {
//             // Wasm builds will check for meta files (that don't exist) if this isn't set.
//             // This causes errors and even panics in web builds on itch.
//             // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
//             meta_check: AssetMetaCheck::Never,
//             ..default()
//         }))
//         .add_systems(Startup, setup)
//         .run();
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn(SpriteBundle {
//         texture: asset_server.load("ducky.png"),
//         ..Default::default()
//     });
// }

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;

mod helpers;

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
        .add_systems(Startup, startup)
        .insert_resource(LevelSelection::index(0))
        .run();

    // App::new()
    //     .add_plugins()
    //     .add_plugins(TilemapPlugin)
    //     .add_plugins(helpers::LdtkPlugin)
    //     .add_systems(Startup, startup)
    //     .add_systems(Update, helpers::movement)
    //     .run();
}
