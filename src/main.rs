mod camera;
mod player;
mod ui;
mod world;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::camera::CameraManagerPlugin;
use crate::player::PlayerManagerPlugin;
use crate::ui::UiManagerPlugin;
use crate::world::WorldManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraManagerPlugin)
        .add_plugins(PlayerManagerPlugin)
        .add_plugins(UiManagerPlugin)
        .add_plugins(WorldManagerPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .run();
}
