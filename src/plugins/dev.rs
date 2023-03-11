use crate::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    #[allow(unused)]
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin);
    }
}
