use crate::{consts, orbs::Orb, player::Player};
use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_light_2d::light::{AmbientLight2d, PointLight2d, PointLight2dBundle};

use crate::game_state::{GameState, TimeState};

pub struct LightPlugin;

#[derive(Component)]
struct OrbLight;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_daylight, add_orb_lights, add_player_light)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_daylight(time_state: Res<TimeState>, mut ambient_light: Query<&mut AmbientLight2d>) {
    for mut light in &mut ambient_light {
        let hour = time_state.current_hour();
        let daylight = consts::BASE_LIGHT
            + 0.8 * 0.5 * (((hour as f32 - 12.0) * (2.0 * 3.141) / 24.0).cos() + 1.0);
        light.brightness = daylight;
    }
}

fn add_player_light(
    mut commands: Commands,
    query: Query<Entity, (With<Player>, Without<OrbLight>)>,
) {
    for player in &query {
        let light = commands
            .spawn((PointLight2dBundle {
                point_light: PointLight2d {
                    radius: 90.0,
                    color: Color::Srgba(WHITE),
                    intensity: 10.0,
                    falloff: 20.0,
                },
                ..default()
            },))
            .id();
        commands.entity(player).add_child(light);
        commands.entity(player).insert(OrbLight);
    }
}

fn add_orb_lights(mut commands: Commands, query: Query<Entity, (With<Orb>, Without<OrbLight>)>) {
    for orb in &query {
        let light = commands
            .spawn((PointLight2dBundle {
                point_light: PointLight2d {
                    radius: 90.0,
                    color: Color::Srgba(WHITE),
                    intensity: 25.0,
                    falloff: 30.0,
                },
                ..default()
            },))
            .id();
        commands.entity(orb).add_child(light);
        commands.entity(orb).insert(OrbLight);
    }
}
