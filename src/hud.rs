use bevy::prelude::*;

use crate::game_state::{GameState, TimeState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnExit(GameState::Playing), despawn_hud)
            .add_systems(
                Update,
                (
                    update_hour_indicator,
                    update_seasion_indicator,
                    update_time_advance_indicator,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Hud;

#[derive(Component, Default)]
pub struct LevelName;

#[derive(Component, Default)]
pub struct HourIndicator;

#[derive(Component, Default)]
pub struct SeasonIndicator;

#[derive(Component, Default)]
pub struct TimeAdvanceIndicator;

fn spawn_hud(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };

    commands.spawn((container, Hud)).with_children(|hud| {
        // TOP Area
        hud.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect {
                    left: Val::Px(30.0),
                    right: Val::Px(30.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|top_area| {
            let level_name = TextBundle::from_section(
                "level 1",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            );

            let hour_indicator = TextBundle::from_section(
                "1:00",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            );

            let seasion_indicator = TextBundle::from_section(
                "winter",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            );

            top_area.spawn((hour_indicator, HourIndicator));
            top_area.spawn((level_name, LevelName));
            top_area.spawn((seasion_indicator, SeasonIndicator));
        });

        hud.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|bottom_area| {
            let time_advance = TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            );
            bottom_area.spawn((time_advance, TimeAdvanceIndicator));
        });
    });
}

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    let hud = commands.entity(query.single());
    hud.despawn_recursive();
}

pub fn update_seasion_indicator(
    time_state: Res<TimeState>,
    mut seasion_indicator: Query<&mut Text, With<SeasonIndicator>>,
) {
    let mut seasion_indicator = seasion_indicator.single_mut();
    seasion_indicator.sections[0].value = format!("{}", time_state.current_seasion());
}

pub fn update_hour_indicator(
    time_state: Res<TimeState>,
    mut hour_indictor: Query<&mut Text, With<HourIndicator>>,
) {
    let mut hour_indicator = hour_indictor.single_mut();
    hour_indicator.sections[0].value = format!("{}:00", time_state.current_hour());
}

pub fn update_time_advance_indicator(
    time_state: Res<TimeState>,
    mut time_advance_indicator: Query<&mut Text, With<TimeAdvanceIndicator>>,
) {
    let mut time_advance_indicator = time_advance_indicator.single_mut();
    time_advance_indicator.sections[0].value = format!(
        "Time will move {} hours when you move in the {} direction",
        time_state.time_step_delta, time_state.time_axis
    );
}
