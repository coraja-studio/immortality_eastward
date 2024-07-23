use crate::game::ui::percentage::{AsPercentage, Percentage};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, process_damage_events)
        .add_systems(Update, handle_death)
        .add_event::<DamageEvent>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub hit_points: f32,
    pub max: f32,
}

impl AsPercentage for Health {
    fn percentage(&self) -> Percentage {
        Percentage::new(self.hit_points / self.max)
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Health {
            hit_points: max,
            max,
        }
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub damage: f32,
    pub target: Entity,
    pub source: Entity,
}

pub fn process_damage_events(
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for &DamageEvent { damage, target, .. } in events.read() {
        if let Ok(mut health) = health_query.get_mut(target) {
            health.hit_points -= damage;
        }
    }
}

fn handle_death(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.hit_points <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
