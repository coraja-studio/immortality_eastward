use bevy::prelude::*;

use super::health::Health;
use status_bar::plugin::StatusBarPlugin;

pub mod percentage;
pub mod status_bar;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(StatusBarPlugin::<Health>::default());
}
