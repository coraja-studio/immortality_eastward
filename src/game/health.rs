use crate::game::ui::percentage::{AsPercentage, Percentage};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_death);
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

fn handle_death(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.hit_points <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
