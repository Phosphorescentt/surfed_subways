use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::toml::TomlAssetPlugin;

pub struct PatternManagerPlugin;

impl Plugin for PatternManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<Pattern>::new(&["pattern.toml"]))
            .add_systems(Startup, (load_patterns, validate_patterns).chain());
    }
}

struct PatternStore {
    size: i32,
    hashmap: HashMap<String, Pattern>,
}

#[derive(serde::Deserialize, Debug)]
struct PatternMetadata {
    unique_name: String,
    name: String,
    version: String,
}

#[derive(serde::Deserialize, Debug)]
struct PatternData {
    leads_from: Vec<String>,
    leads_to: Vec<String>,
    pattern: Vec<PatternRow>,
}

#[derive(serde::Deserialize, Debug)]
struct PatternRow {
    left: ObjectType,
    middle: ObjectType,
    right: ObjectType,
}

#[derive(serde::Deserialize, Debug)]
enum ObjectType {
    #[serde(alias = "")]
    #[serde(alias = "e")]
    #[serde(alias = "n")]
    None,
    #[serde(alias = "c")]
    CoinLow,
    #[serde(alias = "C")]
    CoinHigh,
    #[serde(alias = "r")]
    Ramp,
    #[serde(alias = "b")]
    Barrier,
    #[serde(alias = "a")]
    CoinArcStartLow,
    #[serde(alias = "x")]
    CoinArcEndLow,
    #[serde(alias = "A")]
    CoinArcStartHigh,
    #[serde(alias = "X")]
    CoinArcEndHigh,
    #[serde(alias = "p")]
    PowerupSpawnLow,
    #[serde(alias = "P")]
    PowerupSpawnHigh,
}

#[derive(serde::Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Pattern {
    metadata: PatternMetadata,
    #[serde(alias = "pattern_data")]
    data: PatternData,
}

fn load_patterns(mut commands: Commands, server: Res<AssetServer>) {
    // collect all patterns from filesystem and parse them into
    // Rust objects.

    info!("Loading patterns...");
    let handles = server.load_folder("patterns");
    match handles {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}

fn validate_patterns(patterns: Res<Assets<Pattern>>) {
    // validate patterns (i.e. ensure nothing happens between a
    // CoinArcStart and a CoinArcStop or that they're not too
    // far apart)

    info!("Validating patterns...");
    for pattern in patterns.iter() {
        println!("pattern: {:?}", pattern);
    }

    // check all patterns are compatible (i.e. no dead ends)
}
