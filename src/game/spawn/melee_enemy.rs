//! Spawn the melee enemies.

use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        behaviour::follow::FollowPlayer,
        movement::{Movement, MovementController},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_melee_enemies);
    app.register_type::<MeleeEnemy>();
}

#[derive(Event, Debug)]
pub struct SpawnMeleeEnemy;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct MeleeEnemy;

fn spawn_melee_enemies(
    _trigger: Trigger<SpawnMeleeEnemy>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our melee enemies. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for index in 0..10 {
        let animation = PlayerAnimation::new();
        let translation: Vec3 = Vec3::new((100.0 * index as f32) - 500.0, 150.0, 1.0);
        commands.spawn((
            Name::new("MeleeEnemy"),
            MeleeEnemy,
            SpriteBundle {
                texture: image_handles[&ImageKey::EvilDucky].clone_weak(),
                transform: Transform {
                    translation: translation,
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation.get_atlas_index(),
            },
            MovementController::default(),
            Movement { speed: 100.0 },
            FollowPlayer {
                until_distance: 5.0,
            },
            animation,
            StateScoped(Screen::Playing),
        ));
    }
}
