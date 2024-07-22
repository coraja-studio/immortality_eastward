//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use super::input::PlayerAction;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::spawn::player::Player;
use crate::AppSet;

/// Camera lerp factor.
const CAM_LERP_FACTOR: f32 = 2.;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );
    // Apply movement based on controls.
    app.register_type::<Movement>();
    app.add_systems(Update, apply_movement.in_set(AppSet::Update));
    app.add_systems(PostUpdate, update_camera);
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn record_movement_controller(
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;

    for mut movement_controller in &mut query {
        // When the default input for `PlayerAction::Run` is pressed, print the clamped direction of the axis
        if action_state.pressed(&PlayerAction::Move) {
            intent = action_state
                .clamped_axis_pair(&PlayerAction::Move)
                .unwrap()
                .xy();
        }

        // When the default input for `PlayerAction::Interact` is pressed, print "Interact!"
        if action_state.just_pressed(&PlayerAction::Interact) {
            println!("Interact!");
        }

        // Normalize so that diagonal movement has the same speed as
        // horizontal and vertical movement.
        if intent.length_squared() > 1.0 {
            intent = intent.normalize_or_zero();
        }

        // Apply movement intent to controllers.
        movement_controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    pub controls_rigid_body: bool,
    pub speed: f32,
}

impl Movement {
    pub fn new(speed: f32) -> Movement {
        Movement {
            controls_rigid_body: true,
            speed,
        }
    }

    pub fn toggle_control(&mut self, toggle: bool) {
        self.controls_rigid_body = toggle;
    }
}

fn apply_movement(
    mut movement_query: Query<(&MovementController, &Movement, &mut LinearVelocity)>,
) {
    for (controller, movement, mut linear_velocity) in &mut movement_query {
        if movement.controls_rigid_body {
            let velocity = movement.speed * controller.0;
            linear_velocity.0 = velocity;
        }
    }
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, .. } = player.translation;
    let direction = Vec3::new(x, camera.translation.y, camera.translation.z);

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    camera.translation = camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}
