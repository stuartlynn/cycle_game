use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game_state::GameState;

pub struct OrbsPlugin;

#[derive(Component, Default)]
pub struct Orb;

#[derive(Component, Deref, DerefMut)]
pub struct OrbTimer(Timer);

impl Default for OrbTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Component, Default)]
pub struct AxisSwitch;

#[derive(Default, Bundle, LdtkEntity)]
struct AxisSwitchBundle {
    axis_switch: AxisSwitch,
    orb: Orb,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    animation_timer: OrbTimer,
    anim_frame: OrbAnimFrame,
}

#[derive(Component, Default)]
pub struct DirectionSwitch;

#[derive(Default, Bundle, LdtkEntity)]
struct DirectionSwitchBundle {
    direction_switch: DirectionSwitch,
    orb: Orb,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    animation_timer: OrbTimer,
    anim_frame: OrbAnimFrame,
}

#[derive(Component, Default)]
pub struct SpeedUp;

#[derive(Default, Bundle, LdtkEntity)]
struct SpeedUpBundle {
    speed_up: SpeedUp,
    orb: Orb,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    animation_timer: OrbTimer,
    anim_frame: OrbAnimFrame,
}

#[derive(Component, Default)]
pub struct SlowDown;

#[derive(Default, Bundle, LdtkEntity)]
struct SlowDownBundle {
    speed_up: SpeedUp,
    orb: Orb,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    animation_timer: OrbTimer,
    anim_frame: OrbAnimFrame,
}

#[derive(Default, Component)]
struct OrbAnimFrame {
    pub base: Option<usize>,
    pub frame: usize,
}

impl OrbAnimFrame {
    pub fn advance(&mut self) {
        self.frame = (self.frame + 1) % 4;
    }
}

impl Plugin for OrbsPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<AxisSwitchBundle>("Axis_switch")
            .register_ldtk_entity::<DirectionSwitchBundle>("Direction_switch")
            .register_ldtk_entity::<SpeedUpBundle>("Speed_up")
            .register_ldtk_entity::<SlowDownBundle>("Slow_down")
            .add_systems(Update, (animate_orbs).run_if(in_state(GameState::Playing)));
    }
}

fn animate_orbs(
    time: Res<Time>,
    mut orbs: Query<(&mut TextureAtlas, &mut OrbAnimFrame, &mut OrbTimer), With<Orb>>,
) {
    for (mut atlas, mut frame, mut timer) in &mut orbs {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(base) = frame.base {
                atlas.index = base + frame.frame;
            } else {
                frame.base = Some(atlas.index);
            }

            frame.advance()
        }
    }
}
