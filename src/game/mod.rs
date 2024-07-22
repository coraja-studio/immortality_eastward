//! Game mechanics and content.

use avian2d::{prelude::*, PhysicsPlugins};
use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod behaviour;
mod damaging_contacts;
pub mod dash;
pub mod health;
pub mod input;
mod kinematic_controller_collisions;
mod movement;
pub mod spawn;
pub mod ui;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    PlayerMovement,
    PlayerHitbox,
    Enemies,
    LevelBounds,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        animation::plugin,
        audio::plugin,
        assets::plugin,
        damaging_contacts::plugin,
        dash::plugin,
        input::plugin,
        movement::plugin,
        spawn::plugin,
        behaviour::plugin,
        health::plugin,
        kinematic_controller_collisions::plugin,
        ui::plugin,
    ));
}
