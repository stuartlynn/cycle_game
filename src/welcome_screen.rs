use bevy::prelude::*;

use crate::game_state::GameState;

pub struct WelcomeScreenPlugin;

impl Plugin for WelcomeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::WelcomeScreen), spawn_welcome_screen)
            .add_systems(OnExit(GameState::WelcomeScreen), despawn_welcome_screen)
            .add_systems(
                Update,
                menu_screen_key_press.run_if(in_state(GameState::WelcomeScreen)),
            );
    }
}

#[derive(Component)]
pub struct WelcomeScreen;

pub fn menu_screen_key_press(
    mut next_state: ResMut<NextState<GameState>>,

    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing)
    }
}

pub fn spawn_welcome_screen(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let text = commands
        .spawn(TextBundle::from_sections([
            TextSection::new(
                "CycleGame!",
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Press any key to start!",
                TextStyle {
                    font_size: 75.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ]))
        .id();

    let parent = commands.spawn((container, WelcomeScreen)).id();

    commands.entity(parent).push_children(&[text]);
}

pub fn despawn_welcome_screen(mut commands: Commands, query: Query<Entity, With<WelcomeScreen>>) {
    let intro_screen = commands.entity(query.single());
    intro_screen.despawn_recursive();
}
