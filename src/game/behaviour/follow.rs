use bevy::prelude::*;
use crate::game::spawn::player::Player;
use crate::game::movement::MovementController;
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FollowPlayer>();
    app.add_systems(
        Update,
        follow_player.in_set(AppSet::RecordInput),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FollowPlayer {
    pub until_distance: f32,
}

pub fn follow_player(
    mut query: Query<(&FollowPlayer, &mut MovementController, &Transform)>,
    player_query: Query<&Transform, With<Player>>) {
        let Ok(player_transform) = player_query.get_single() else {
            return;
        };
        for (follow_player,
            mut movement_controller,
            transform,
        ) in &mut query {
            let player_direction: Vec2 = (player_transform.translation - transform.translation).xy();
            if player_direction.length_squared() <= follow_player.until_distance {
                movement_controller.0 = Vec2::ZERO;
                continue;
            }
            movement_controller.0 = player_direction.normalize_or_zero();
        }
}
