use bevy::prelude::*;

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditGameState;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn emit_edit_game_state(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut write_edit_game_state: EventWriter<EditGameState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        write_edit_game_state.send(EditGameState {});
        keyboard_input.reset(KeyCode::Space);
    }
}

pub fn handle_edit_game_state(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut read_edit_game_state: EventReader<EditGameState>,
) {
    for _ in read_edit_game_state.read() {
        match *game_state.get() {
            GameState::Playing => {
                commands.insert_resource(NextState(Some(GameState::Paused)));
                info!("GameState change: Playing -> Paused");
            }
            GameState::Paused => {
                commands.insert_resource(NextState(Some(GameState::Playing)));
                info!("GameState change: Paused -> Playing");
            }
        }
    }
}
