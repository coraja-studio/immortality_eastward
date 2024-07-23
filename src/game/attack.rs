use std::time::Duration;

use super::{
    animation::AttackAnimation,
    assets::{HandleMap, ImageKey},
    damage_zone::DamageZoneBundle,
    input::PlayerAction,
    GameLayer,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::spawn::player::Player;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AttackController>();
    app.add_systems(Update, record_attack_controller.in_set(AppSet::RecordInput));
    app.register_type::<Attack>();
    app.add_systems(Update, apply_attack.in_set(AppSet::Update));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackController {
    pub look_direction: Vec2,
    pub intent: bool,
}

fn record_attack_controller(
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut AttackController, With<Player>>,
) {
    for mut attack_controller in &mut query {
        let mut look_intent = Vec2::ZERO;
        if action_state.pressed(&PlayerAction::Look) {
            look_intent = action_state.axis_pair(&PlayerAction::Look).unwrap().xy();
        }
        if look_intent.length_squared() > 0.05 {
            attack_controller.look_direction = look_intent.normalize_or_zero();
        }

        attack_controller.intent = action_state.just_pressed(&PlayerAction::AttackPrimary)
            || action_state.just_pressed(&PlayerAction::AttackSecondary);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Attack;

fn apply_attack(
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut attack_query: Query<(Entity, &Transform, &AttackController, &Attack)>,
) {
    for (entity, transform, controller, _) in &mut attack_query {
        let attack_transform = Transform {
            translation: transform.translation,
            rotation: Quat::from_rotation_z(Vec2::X.angle_between(controller.look_direction)),
            ..default()
        };

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 1, Some(UVec2::ZERO), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let attack_animation = AttackAnimation::new();

        if controller.intent {
            commands
                .spawn((
                    Name::new("BaseAttack"),
                    DamageZoneBundle::new(
                        entity,
                        10.0,
                        Duration::from_millis(180),
                        32.0,
                        GameLayer::PlayerHitbox,
                        GameLayer::Enemies,
                    ),
                    TransformBundle::from_transform(attack_transform),
                    InheritedVisibility::VISIBLE,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation((Vec2::X * 50.0).extend(10.0)),
                            texture: image_handles[&ImageKey::BaseAttack].clone_weak(),
                            ..default()
                        },
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: attack_animation.get_atlas_index(),
                        },
                        attack_animation,
                    ));
                });
        }
    }
}
