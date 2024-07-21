//! Game mechanics and content.

use avian2d::{prelude::*, PhysicsPlugins};
use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod behaviour;
pub mod input;
mod kinematic_controller_collisions;
mod movement;
pub mod spawn;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Enemies,
    LevelBounds,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        animation::plugin,
        audio::plugin,
        assets::plugin,
        input::plugin,
        movement::plugin,
        spawn::plugin,
        behaviour::plugin,
        kinematic_controller_collisions::plugin,
    ));
}
