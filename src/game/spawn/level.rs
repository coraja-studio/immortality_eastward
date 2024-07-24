//! Spawn the main level by triggering other observers.

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game::GameLayer;

use super::{melee_enemy::SpawnMeleeEnemy, player::SpawnPlayer};

pub const LEVEL_HEIGHT: f32 = 700.0;
const WALL_THICKNESS: f32 = 50.0;

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

    let square_sprite = Sprite {
        color: Color::srgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(WALL_THICKNESS)),
        ..default()
    };
    // Ceiling
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, (LEVEL_HEIGHT + WALL_THICKNESS) * 0.5, 0.0)
                .with_scale(Vec3::new(2000.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, WALL_THICKNESS),
        CollisionLayers::new(
            GameLayer::LevelBounds,
            [GameLayer::Enemies, GameLayer::PlayerMovement],
        ),
    ));
    // Floor
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, -(LEVEL_HEIGHT + WALL_THICKNESS) * 0.5, 0.0)
                .with_scale(Vec3::new(2000.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WALL_THICKNESS, WALL_THICKNESS),
        CollisionLayers::new(
            GameLayer::LevelBounds,
            [GameLayer::Enemies, GameLayer::PlayerMovement],
        ),
    ));

    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnMeleeEnemy);
}
