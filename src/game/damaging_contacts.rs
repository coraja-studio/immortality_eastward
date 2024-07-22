use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    health::{DamageEvent, Health},
    spawn::{
        melee_enemy::MeleeEnemy,
        player::{Player, PlayerHitBox},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_damaging_contacts);
}

fn handle_damaging_contacts(
    mut events: EventWriter<DamageEvent>,
    query: Query<&CollidingEntities, With<MeleeEnemy>>,
    player_query: Query<Entity, (With<Health>, With<Player>)>,
    player_hit_box_query: Query<Entity, With<PlayerHitBox>>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    let Ok(player_hitbox_entity) = player_hit_box_query.get_single() else {
        return;
    };

    for enemy_colliding_entities in &query {
        if enemy_colliding_entities.0.contains(&player_hitbox_entity) {
            events.send(DamageEvent{damage: 1.0, target: player_entity});
        }
    }
}
