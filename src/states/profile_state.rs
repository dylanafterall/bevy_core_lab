use bevy::prelude::*;

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ProfileState {
    #[default]
    Profile1,
    Profile2,
    Profile3,
    Profile4,
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditProfileState {
    pub desired_profile_state: ProfileState,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn handle_edit_profile_state(
    mut next_profile_state: ResMut<NextState<ProfileState>>,
    mut read_edit_profile_state: EventReader<EditProfileState>,
) {
    for profile_state_edit in read_edit_profile_state.read() {
        next_profile_state.set(profile_state_edit.desired_profile_state);
        info!(
            "ProfileState change to: {:?}",
            profile_state_edit.desired_profile_state
        );
    }
}
