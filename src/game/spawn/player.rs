//! Spawn the player.

use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
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

    commands
        .spawn((
            Name::new("Player"),
            Player,
            SpriteBundle {
                texture: image_handles[&ImageKey::ImmortalitySeeker2].clone_weak(),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            MovementController::default(),
            Movement::new(200.0),
            player_animation,
            StateScoped(Screen::Playing),
            RigidBody::Kinematic,
            Collider::circle(10.0),
            CollisionLayers::new(GameLayer::PlayerMovement, GameLayer::LevelBounds),
            Health::new(200.0),
            StatusBarDefinition::<Health>::default(),
            DashController::new(),
            Dash::new(
                600.0,
                Duration::from_millis(200),
                Duration::from_millis(100),
                Duration::new(2, 0),
            ),
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
