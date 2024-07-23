use std::{collections::HashSet, time::Duration};

use avian2d::prelude::*;
use bevy::prelude::*;

use super::health::{DamageEvent, Health};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_damage_zones);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DamageZone {
    emitter: Entity,
    damage: f32,
    lifetime: Duration,
}

/// A component that stores the entities that were damaged by damage zone already.
#[derive(Reflect, Clone, Component, Debug, Default, Deref, DerefMut, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", reflect(Serialize, Deserialize))]
#[reflect(Debug, Component, Default, PartialEq)]
pub struct DamagedEntities(pub HashSet<Entity>);

#[derive(Bundle)]
pub struct DamageZoneBundle {
    pub damage_zone: DamageZone,
    pub collider: Collider,
    pub layers: CollisionLayers,
    pub sensor: Sensor,
    pub damaged_entities: DamagedEntities,
}

impl DamageZoneBundle {
    pub fn new(
        emitter: Entity,
        damage: f32,
        lifetime: Duration,
        circle_radius: f32,
        own_layer: impl Into<LayerMask>,
        collides_with_layers: impl Into<LayerMask>,
    ) -> DamageZoneBundle {
        DamageZoneBundle {
            damage_zone: DamageZone {
                emitter,
                damage,
                lifetime,
            },
            collider: Collider::rectangle(circle_radius * 2.0, circle_radius),
            layers: CollisionLayers::new(own_layer, collides_with_layers),
            sensor: Sensor,
            damaged_entities: DamagedEntities(HashSet::new()),
        }
    }
}

fn handle_damage_zones(
    mut commands: Commands,
    mut events: EventWriter<DamageEvent>,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &CollidingEntities,
        &mut DamageZone,
        &mut DamagedEntities,
    )>,
    damageable_query: Query<Entity, With<Health>>,
) {
    for (damage_zone_entity, colliding_entities, mut damage_zone, mut damaged_entities) in
        &mut query
    {
        damage_zone.lifetime = damage_zone.lifetime.saturating_sub(time.delta());
        if damage_zone.lifetime <= Duration::ZERO {
            commands.entity(damage_zone_entity).despawn_recursive();
            continue;
        }

        for &colliding_entity in colliding_entities.iter() {
            if damaged_entities.0.contains(&colliding_entity) {
                continue;
            }

            let Ok(_) = damageable_query.get(colliding_entity) else {
                continue;
            };

            damaged_entities.0.insert(colliding_entity);
            events.send(DamageEvent {
                damage: damage_zone.damage,
                target: colliding_entity,
                source: damage_zone.emitter,
            });
        }
    }
}
