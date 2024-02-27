#![allow(unused)]
// TODO: remove allow(unused) upon implementing persistent data functionality

use bevy::prelude::*;

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DataState {
    Idle,
    #[default]
    Loading, // assets
    Saving, // scenes
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditDataState {
    pub desired_data_state: DataState,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn handle_edit_data_state(
    mut next_data_state: ResMut<NextState<DataState>>,
    mut read_edit_data_state: EventReader<EditDataState>,
) {
    for data_state_edit in read_edit_data_state.read() {
        next_data_state.set(data_state_edit.desired_data_state);
        info!(
            "DataState change to: {:?}",
            data_state_edit.desired_data_state
        );
    }
}
