mod camera;
mod pattern;
mod player;
mod ui;
mod world;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::camera::CameraManagerPlugin;
use crate::pattern::PatternManagerPlugin;
use crate::player::PlayerManagerPlugin;
use crate::ui::UiManagerPlugin;
use crate::world::WorldManagerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraManagerPlugin,
            PatternManagerPlugin,
            PlayerManagerPlugin,
            UiManagerPlugin,
            WorldManagerPlugin,
            WorldInspectorPlugin::default(),
        ))
        .run();
}
