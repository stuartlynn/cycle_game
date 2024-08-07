use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "inspector")]
        {
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }
    }
}
