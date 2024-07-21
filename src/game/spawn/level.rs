//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::{melee_enemy::SpawnMeleeEnemy, player::SpawnPlayer};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/TX Tileset Grass.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(5000.0, 2000.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec2::splat(0.0).extend(-1.0),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0,
        },
    ));
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnMeleeEnemy);
}
