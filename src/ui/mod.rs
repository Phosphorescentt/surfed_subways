use crate::world::Score;
use bevy::prelude::*;

pub struct UiManagerPlugin;

impl Plugin for UiManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            transform: Transform::default(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Text Example",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ))
                .insert(ScoreText);
        })
        .insert(Name::new("UI"));
}

fn update_ui(mut query: Query<&mut Text, With<ScoreText>>, score_res: Res<Score>) {
    for mut text in &mut query {
        text.sections[0].value = format!("Coins: {}, Score: {}", score_res.coins, score_res.score);
    }
}
