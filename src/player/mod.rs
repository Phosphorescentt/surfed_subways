use std::time::Duration;

use crate::world::{Coin, Lane, LaneObject, Score};
use bevy::prelude::*;

pub struct PlayerManagerPlugin;

impl Plugin for PlayerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (move_player, collect_coins));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerMoveCooldownTime {
    timer: Timer,
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(0.25, 1.5, 0.75).into()),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_xyz(-15.0, 1.0, 0.0),
            ..default()
        })
        .insert(Player)
        .insert(LaneObject { lane: Lane::MIDDLE })
        .insert(PlayerMoveCooldownTime {
            timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
        })
        .insert(Name::new("Player"));
}

fn move_player(
    mut player_query: Query<
        (&mut Transform, &mut LaneObject, &mut PlayerMoveCooldownTime),
        With<Player>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_transform, mut player_lane_obj, mut move_cooldown_timer) =
        player_query.single_mut();

    move_cooldown_timer.timer.tick(time.delta());

    if move_cooldown_timer.timer.finished() {
        match player_lane_obj.lane {
            Lane::LEFT => {
                if keyboard_input.just_pressed(KeyCode::D) {
                    player_transform.translation.z = 0.0;
                    player_lane_obj.lane = Lane::MIDDLE;
                    move_cooldown_timer.timer.reset();
                }
            }
            Lane::MIDDLE => {
                if keyboard_input.just_pressed(KeyCode::A) {
                    player_transform.translation.z = -1.0;
                    player_lane_obj.lane = Lane::LEFT;
                    move_cooldown_timer.timer.reset();
                }
                if keyboard_input.just_pressed(KeyCode::D) {
                    player_transform.translation.z = 1.0;
                    player_lane_obj.lane = Lane::RIGHT;
                    move_cooldown_timer.timer.reset();
                }
            }
            Lane::RIGHT => {
                if keyboard_input.just_pressed(KeyCode::A) {
                    player_transform.translation.z = 0.0;
                    player_lane_obj.lane = Lane::MIDDLE;
                    move_cooldown_timer.timer.reset();
                }
            }
        };
    }
}

fn collect_coins(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    mut coin_query: Query<(&Transform, Entity), With<Coin>>,
    mut score_res: ResMut<Score>,
) {
    let player_transform = player_query.single_mut();

    for (coin_transform, entity) in coin_query.iter_mut() {
        let distance = coin_transform
            .translation
            .distance(player_transform.translation);
        if distance < 1.0 {
            commands.entity(entity).despawn_recursive();
            score_res.coins += 1;
        }
    }
}
