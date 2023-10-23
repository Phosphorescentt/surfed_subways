use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;
use std::{f32::consts::PI, time::Duration};

const SPEED: f32 = 10.0;

#[derive(Component)]
struct Scroller;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, (setup_track, setup_camera, setup_light))
        .add_systems(FixedUpdate, (move_scrollers, cull_stuff_behind_camera))
        .add_systems(
            Update,
            (
                spawn_track_lines.run_if(on_timer(Duration::from_secs(1))),
                spawn_coins.run_if(on_timer(Duration::from_millis(500))),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 2.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_light(mut commands: Commands) {
    // TODO: Fix light colour, this is washing everything out.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::splat(0.0),
            // rotation: Quat::from_rotation_x(-PI / 4.),
            // rotation: Quat::from_vec4(Vec4::splat(-PI / 4.)),
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 3., -PI / 6., 0.),
            ..default()
        },
        ..default()
    });
}

fn setup_track(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = shape::Box::new(200.0, 0.1, 1.0);
    let x_transform: f32 = 50.0;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.into()),
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            transform: Transform::from_xyz(x_transform, 0.0, -1.0),
            ..default()
        })
        .insert(Name::new("Left Lane"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.into()),
            material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
            transform: Transform::from_xyz(x_transform, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Middle Lane"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.into()),
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_xyz(x_transform, 0.0, 1.0),
            ..default()
        })
        .insert(Name::new("Right Lane"));
}

fn spawn_track_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = shape::Box::new(0.1, 0.1, 3.0);

    // TODO: Fix this. This will spawn track lines every second so it feels like
    // you're always going the same speed when in reality you're going much quicker.
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.into()),
            material: materials.add(Color::DARK_GRAY.into()),
            transform: Transform::from_xyz(20.0, 0.1, 0.0),
            ..default()
        })
        .insert(Scroller)
        .insert(Name::new("Track Line"));
}

fn move_scrollers(mut query: Query<&mut Transform, With<Scroller>>, time: Res<FixedTime>) {
    for mut scroller_transform in query.iter_mut() {
        scroller_transform.translation +=
            Vec3::new(-1.0, 0.0, 0.0) * SPEED * time.period.as_secs_f32();
    }
}

fn cull_stuff_behind_camera(
    mut commands: Commands,
    mut scroller_query: Query<(Entity, &mut Transform), With<Scroller>>,
) {
    for (entity, scroller_transform) in scroller_query.iter_mut() {
        if scroller_transform.translation.x < -20.0 {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn spawn_coins(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let mesh = shape::Cube::new(0.5);
    let material = Color::YELLOW;
    let lane_number: i16 = (rng.gen::<i16>()).rem_euclid(3) - 1;
    let transform = Transform::from_xyz(20., 0.5, lane_number as f32);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh.into()),
            material: materials.add(material.into()),
            transform,
            ..default()
        })
        .insert(Scroller)
        .insert(Name::new("Coin"));
}
