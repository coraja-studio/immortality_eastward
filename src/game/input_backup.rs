use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PlayerAction{
    Run,
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
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(Self::Run, DualAxis::left_stick());
        input_map.insert(Self::Look, DualAxis::right_stick());
        input_map.insert(Self::Dash, GamepadButtonType::South);
        input_map.insert(Self::Interact, GamepadButtonType::West);
        input_map.insert(Self::AttackPrimary, GamepadButtonType::LeftTrigger2);
        input_map.insert(Self::AttackSecondary, GamepadButtonType::RightTrigger2);

        // Default kbm input bindings
        input_map.insert(Self::Run, VirtualDPad::wasd());
        input_map.insert(Self::Dash, KeyCode::Space);
        input_map.insert(Self::Interact, KeyCode::KeyE);
        input_map.insert(Self::AttackPrimary, MouseButton::Left);
        input_map.insert(Self::AttackSecondary, MouseButton::Right);

        input_map
    }
}

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.init_resource::<ActionState<PlayerAction>>();
    app.insert_resource(PlayerAction::default_input_map());
}
