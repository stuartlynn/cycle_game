use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    player: Goal,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<GoalBundle>("Goal");
    }
}
