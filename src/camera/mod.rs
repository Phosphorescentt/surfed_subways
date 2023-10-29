use bevy::prelude::*;

pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 2.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
