use super::{app_state, data_state, game_state, options_state, profile_state};

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
            .init_state::<data_state::DataState>()
            .init_state::<profile_state::ProfileState>()
            .add_event::<app_state::EditAppState>()
            .add_event::<game_state::EditGameState>()
            .add_event::<options_state::EditOptionsState>()
            .add_event::<data_state::EditDataState>()
            .add_event::<profile_state::EditProfileState>()
            .add_systems(
                Update,
                (
                    app_state::handle_edit_app_state
                        .after(profile_state::handle_edit_profile_state),
                    game_state::emit_edit_game_state,
                    game_state::handle_edit_game_state.run_if(in_state(app_state::AppState::Game)),
                    options_state::handle_edit_options_state,
                    data_state::handle_edit_data_state
                        .run_if(in_state(app_state::AppState::Game))
                        .run_if(in_state(app_state::AppState::Options)),
                    profile_state::handle_edit_profile_state
                        .run_if(in_state(app_state::AppState::LoadSave)),
                ),
            );
    }
}
