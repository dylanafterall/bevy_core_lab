use bevy::prelude::*;

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Resource, Debug)]
pub struct OptionsStateAllowedChanges {
    pub none: Vec<OptionsState>,
    pub menu: Vec<OptionsState>,
    pub controls: Vec<OptionsState>,
    pub general: Vec<OptionsState>,
    pub audio: Vec<OptionsState>,
    pub video: Vec<OptionsState>,
}

impl Default for OptionsStateAllowedChanges {
    fn default() -> OptionsStateAllowedChanges {
        OptionsStateAllowedChanges {
            none: vec![OptionsState::Menu],
            menu: vec![
                OptionsState::None,
                OptionsState::General,
                OptionsState::Controls,
                OptionsState::Audio,
                OptionsState::Video,
            ],
            controls: vec![OptionsState::Menu],
            general: vec![OptionsState::Menu],
            audio: vec![OptionsState::Menu],
            video: vec![OptionsState::Menu],
        }
    }
}

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum OptionsState {
    #[default]
    None,
    Menu,
    Controls,
    General,
    Audio,
    Video,
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditOptionsState {
    pub desired_options_state: OptionsState,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn handle_edit_options_state(
    allowed_changes: Res<OptionsStateAllowedChanges>,
    current_options_state: Res<State<OptionsState>>,
    mut next_options_state: ResMut<NextState<OptionsState>>,
    mut read_edit_options_state: EventReader<EditOptionsState>,
) {
    for options_state_edit in read_edit_options_state.read() {
        // search the vector associated with the current OptionsState in OptionsStateAllowedChanges struct
        //  if that vector contains the desired new OptionsState, then such state change is valid
        if (match current_options_state.get() {
            OptionsState::None => &allowed_changes.none,
            OptionsState::Menu => &allowed_changes.menu,
            OptionsState::Controls => &allowed_changes.controls,
            OptionsState::General => &allowed_changes.general,
            OptionsState::Audio => &allowed_changes.audio,
            OptionsState::Video => &allowed_changes.video,
        })
        .contains(&options_state_edit.desired_options_state)
        {
            next_options_state.set(options_state_edit.desired_options_state);
            info!(
                "OptionsState change: {:?} -> {:?}",
                current_options_state.get(),
                options_state_edit.desired_options_state
            );
        }
    }
}
