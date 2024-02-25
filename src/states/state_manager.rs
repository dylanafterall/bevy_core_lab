use super::{app_state, game_state, options_state};

use bevy::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct StateManagerPlugin;

impl Plugin for StateManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<app_state::AppStateAllowedChanges>()
            .init_resource::<options_state::OptionsStateAllowedChanges>()
            .init_state::<app_state::AppState>()
            .init_state::<game_state::GameState>()
            .init_state::<options_state::OptionsState>()
            .add_event::<app_state::EditAppState>()
            .add_event::<game_state::EditGameState>()
            .add_event::<options_state::EditOptionsState>()
            .add_systems(
                Update,
                (
                    app_state::handle_edit_app_state,
                    game_state::emit_edit_game_state,
                    game_state::handle_edit_game_state.run_if(in_state(app_state::AppState::Game)),
                    options_state::handle_edit_options_state,
                ),
            );
    }
}
