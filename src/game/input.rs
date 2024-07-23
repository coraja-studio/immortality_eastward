//! This is a fairly complete example that implements a twin stick controller.
//!
//! The controller supports both gamepad/MKB inputs and switches between them depending on
//! the most recent input.
//!
//! This example builds on top of several concepts introduced in other examples. In particular,
//! the `default_controls`. `mouse_position`, and `action_state_resource` examples.

use crate::AppSet;
use bevy::{
    input::gamepad::GamepadEvent, input::keyboard::KeyboardInput, prelude::*, window::PrimaryWindow,
};
use leafwing_input_manager::{axislike::DualAxisData, prelude::*};

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
        // Defined below, detects whether MKB or gamepad are active
        .add_plugins(InputModeManagerPlugin)
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::default_input_map())
        // Set up the input processing
        .add_systems(
            Update,
            player_mouse_look
                .in_set(AppSet::PrepareInput)
                .run_if(in_state(ActiveInput::MouseKeyboard)),
        );
}

// ----------------------------- Player Action Input Handling -----------------------------
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Look,
    Dash,
    Interact,
    AttackPrimary,
    AttackSecondary,
    AbilityPrimary,
    AbilitySeconary,
    SwitchWeapon,
}

impl PlayerAction {
    /// Define the default binding to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Look, DualAxis::right_stick());
        input_map.insert(Self::Dash, GamepadButtonType::South);
        input_map.insert(Self::Interact, GamepadButtonType::West);
        input_map.insert(Self::AttackPrimary, GamepadButtonType::LeftTrigger2);
        input_map.insert(Self::AttackSecondary, GamepadButtonType::RightTrigger2);

        // Default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Look, VirtualDPad::arrow_keys());
        input_map.insert(Self::Dash, KeyCode::Space);
        input_map.insert(Self::Interact, KeyCode::KeyE);
        input_map.insert(Self::AttackPrimary, MouseButton::Left);
        input_map.insert(Self::AttackSecondary, MouseButton::Right);

        input_map
    }
}

// ----------------------------- Input mode handling -----------------------------
pub struct InputModeManagerPlugin;

impl Plugin for InputModeManagerPlugin {
    fn build(&self, app: &mut App) {
        // Init a state to record the current active input
        app.init_state::<ActiveInput>()
            // System to switch to gamepad as active input
            .add_systems(
                Update,
                activate_gamepad
                .in_set(AppSet::PrepareInput)
                .run_if(in_state(ActiveInput::MouseKeyboard)),
            )
            // System to switch to MKB as active input
            .add_systems(Update, activate_mkb
                .in_set(AppSet::PrepareInput)
                .run_if(in_state(ActiveInput::Gamepad)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ActiveInput {
    #[default]
    MouseKeyboard,
    Gamepad,
}

/// Switch the gamepad when any button is pressed or any axis input used
fn activate_gamepad(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.read() {
        match ev {
            GamepadEvent::Button(_) | GamepadEvent::Axis(_) => {
                info!("Switching to gamepad input");
                next_state.set(ActiveInput::Gamepad);
                return;
            }
            _ => (),
        }
    }
}

/// Switch to mouse and keyboard input when any keyboard button is pressed
fn activate_mkb(
    mut next_state: ResMut<NextState<ActiveInput>>,
    mut kb_evr: EventReader<KeyboardInput>,
) {
    for _ev in kb_evr.read() {
        info!("Switching to mouse and keyboard input");
        next_state.set(ActiveInput::MouseKeyboard);
    }
}

// ----------------------------- Mouse input handling-----------------------------

/// Note that we handle the action state mutation differently here than in the `mouse_position` example.
/// Here we don't use an `ActionStateDriver`, but change the action data directly.
fn player_mouse_look(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut action_state: ResMut<ActionState<PlayerAction>>,
) {
    let (camera_transform, camera) = camera_query.get_single().expect("Need a single camera");
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let window = window_query
        .get_single()
        .expect("Need a single primary window");

    // Many steps can fail here, so we'll wrap in an option pipeline
    // First check if the cursor is in window
    // Then check if the ray intersects the plane defined by the player
    // Then finally compute the point along the ray to look at
    let player_position = player_transform.translation;
    if let Some(cursor_in_world) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .and_then(|ray| {
            Some(ray).zip(ray.intersect_plane(player_position, InfinitePlane3d::new(Vec3::Z)))
        })
        .map(|(ray, p)| ray.get_point(p))
    {
        let diff = (cursor_in_world - player_position).xy();
        if diff.length_squared() > 1e-3f32 {
            // Get the mutable action data to set the axis
            let action_data = action_state.action_data_mut_or_default(&PlayerAction::Look);

            // Flipping y sign here to be consistent with gamepad input.
            // We could also invert the gamepad y-axis
            action_data.axis_pair = Some(DualAxisData::new(diff.x, diff.y));

            // Press the look action, so we can check that it is active
            action_state.press(&PlayerAction::Look);
        }
    }
}
