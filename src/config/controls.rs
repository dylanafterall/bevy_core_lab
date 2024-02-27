use bevy::{
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput},
    prelude::*,
};
use leafwing_input_manager::{
    action_state::ActionState,
    axislike::{DualAxis, VirtualDPad},
    input_map::InputMap,
    prelude::InputManagerPlugin,
    Actionlike,
};

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct MyControlsPlugin;

impl Plugin for MyControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            .init_resource::<ActionState<InputAction>>()
            .insert_resource(InputAction::default_input_map())
            .init_state::<ActiveInput>()
            .add_systems(
                Update,
                (
                    activate_gamepad.run_if(in_state(ActiveInput::MouseKeyboard)),
                    activate_mkb.run_if(in_state(ActiveInput::Gamepad)),
                ),
            );
    }
}

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ActiveInput {
    #[default]
    MouseKeyboard,
    Gamepad,
}

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum InputAction {
    Move,
    Look,
    Select,
}

impl InputAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // default gamepad input bindings
        input_map.insert(Self::Move, DualAxis::left_stick());
        input_map.insert(Self::Look, DualAxis::right_stick());
        input_map.insert(Self::Select, GamepadButtonType::RightTrigger);

        // default kbm input bindings
        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::Look, VirtualDPad::arrow_keys());
        input_map.insert(Self::Select, KeyCode::Space);

        input_map
    }
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
fn activate_gamepad(
    mut next_input_state: ResMut<NextState<ActiveInput>>,
    mut read_gamepad: EventReader<GamepadEvent>,
) {
    for gamepad_event in read_gamepad.read() {
        match gamepad_event {
            GamepadEvent::Button(_) | GamepadEvent::Axis(_) => {
                info!("Switching to gamepad input");
                next_input_state.set(ActiveInput::Gamepad);
                return;
            }
            _ => (),
        }
    }
}

fn activate_mkb(
    mut next_input_state: ResMut<NextState<ActiveInput>>,
    mut read_mkb: EventReader<KeyboardInput>,
) {
    for _ in read_mkb.read() {
        info!("Switching to mouse and keyboard input");
        next_input_state.set(ActiveInput::MouseKeyboard);
    }
}
