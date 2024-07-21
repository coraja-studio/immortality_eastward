use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    health::Health,
    spawn::{
        melee_enemy::MeleeEnemy,
        player::{Player, PlayerHitBox},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_damaging_contacts);
}

fn handle_damaging_contacts(
    query: Query<&CollidingEntities, With<MeleeEnemy>>,
    mut player_query: Query<&mut Health, With<Player>>,
    player_hit_box_query: Query<Entity, With<PlayerHitBox>>,
) {
    let Ok(mut player_health) = player_query.get_single_mut() else {
        return;
    };

    let Ok(player_entity) = player_hit_box_query.get_single() else {
        return;
    };

    for enemy_colliding_entities in &query {
        if enemy_colliding_entities.0.contains(&player_entity) {
            player_health.hit_points -= 1.0;
        }
    }
}
