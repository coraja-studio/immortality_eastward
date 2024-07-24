//! Spawn the player.

use rand::prelude::*;
use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        attack::{Attack, AttackController},
        dash::{Dash, DashController},
        health::Health,
        movement::{Movement, MovementController},
        ui::status_bar::definition::StatusBarDefinition,
        GameLayer,
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerHitBox;

#[derive(Bundle)]
pub struct PlayerMoveCollisionBundle {
    rigid_body: RigidBody,
    collider: Collider,
    collision_layers: CollisionLayers,
}

#[derive(Bundle)]
pub struct PlayerAppearance {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    player_animation: PlayerAnimation,
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::ZERO), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let mut rng = rand::thread_rng();
    let redish = f32::lerp(0.3, 1.0, rng.gen());
    let greenish = f32::lerp(0.3, 1.0, rng.gen());
    let blueish = f32::lerp(0.0, 0.3, rng.gen());

    commands
        .spawn((
            StateScoped(Screen::Playing),
            Name::new("Player"),
            Player,
            PlayerAppearance {
                sprite_bundle: SpriteBundle {
                    texture: image_handles[&ImageKey::ImmortalitySeeker2].clone_weak(),
                    transform: Transform::from_scale(Vec3::splat(2.0)),
                    sprite: Sprite {
                        color: Color::srgb(redish, greenish, blueish),
                        ..default()
                    },
                    ..default()
                },
                texture_atlas: TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: player_animation.get_atlas_index(),
                },
                player_animation,
            },
            MovementController::default(),
            Movement::new(200.0),
            PlayerMoveCollisionBundle {
                rigid_body: RigidBody::Kinematic,
                collider: Collider::circle(10.0),
                collision_layers: CollisionLayers::new(
                    GameLayer::PlayerMovement,
                    GameLayer::LevelBounds,
                ),
            },
            Health::new(200.0),
            StatusBarDefinition::<Health>::default(),
            DashController::new(),
            Dash::new(
                600.0,
                Duration::from_millis(200),
                Duration::from_millis(100),
                Duration::new(2, 0),
            ),
            Attack,
            AttackController {
                look_direction: Vec2::X,
                intent: false,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("PlayerHitbox"),
                PlayerHitBox,
                SpatialBundle::default(),
                Collider::circle(10.0),
                CollisionLayers::new(GameLayer::PlayerHitbox, GameLayer::Enemies),
                Sensor,
            ));
        });
}
