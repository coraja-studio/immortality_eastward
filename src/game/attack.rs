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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct AttackController(pub bool);

fn record_attack_controller(
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut AttackController, With<Player>>,
) {
    for mut attack_controller in &mut query {
        if action_state.just_pressed(&PlayerAction::AttackPrimary) {}
        attack_controller.0 = action_state.just_pressed(&PlayerAction::AttackPrimary)
            || action_state.just_pressed(&PlayerAction::AttackSecondary)
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Attack;

fn apply_attack(
    mut commands: Commands,
    mut attack_query: Query<(Entity, &Transform, &AttackController, &Attack)>,
) {
    for (entity, transform, controller, _) in &mut attack_query {
        let square_sprite = Sprite {
            color: Color::srgba(1.0, 0.0, 0.0, 0.3),
            custom_size: Some(Vec2::splat(30.0)),
            ..default()
        };
        if controller.0 {
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
                    transform: Transform::from_translation(transform.translation),
                    sprite: square_sprite,
                    ..default()
                },
            ));
        }
    }
}
