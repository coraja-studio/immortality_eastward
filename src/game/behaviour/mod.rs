use bevy::prelude::*;
pub mod follow;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(follow::plugin);
}
