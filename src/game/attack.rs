use std::time::Duration;

use super::{damage_zone::DamageZoneBundle, input::PlayerAction, GameLayer};
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
            look_intent = action_state
                .axis_pair(&PlayerAction::Look)
                .unwrap()
                .xy();
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
    mut attack_query: Query<(Entity, &Transform, &AttackController, &Attack)>,
) {
    for (entity, transform, controller, _) in &mut attack_query {
        let square_sprite = Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.3),
            custom_size: Some(Vec2::new(100.0, 50.0)),
            ..default()
        };
        let attack_transform = Transform{
            translation: transform.translation,
            rotation: Quat::from_rotation_z(Vec2::X.angle_between(controller.look_direction)),
            ..default()
        };
        if controller.intent {
            commands.spawn((
                DamageZoneBundle::new(
                    entity,
                    10.0,
                    Duration::from_millis(50),
                    50.0,
                    GameLayer::PlayerHitbox,
                    GameLayer::Enemies,
                ),
                SpriteBundle {
                    transform: attack_transform,
                    sprite: square_sprite,
                    ..default()
                },
            ));
        }
    }
}
