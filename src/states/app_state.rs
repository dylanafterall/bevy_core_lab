use super::options_state::OptionsState;

use bevy::prelude::*;

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Resource, Debug)]
pub struct AppStateAllowedChanges {
    pub splash: Vec<AppState>,
    pub title: Vec<AppState>,
    pub options: Vec<AppState>,
    pub credits: Vec<AppState>,
    pub loadsave: Vec<AppState>,
    pub game: Vec<AppState>,
    pub fail: Vec<AppState>,
}

impl Default for AppStateAllowedChanges {
    fn default() -> AppStateAllowedChanges {
        AppStateAllowedChanges {
            splash: vec![AppState::Title],
            title: vec![AppState::Options, AppState::Credits, AppState::LoadSave],
            options: vec![AppState::Title],
            credits: vec![AppState::Title],
            loadsave: vec![AppState::Title, AppState::Game],
            game: vec![AppState::Fail, AppState::Title],
            fail: vec![AppState::Game, AppState::Title],
        }
    }
}

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Splash,
    Title,
    Options,
    Credits,
    LoadSave,
    Game,
    Fail,
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditAppState {
    pub desired_app_state: AppState,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn handle_edit_app_state(
    current_app_state: Res<State<AppState>>,
    allowed_changes: Res<AppStateAllowedChanges>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut read_edit_app_state: EventReader<EditAppState>,
    mut next_options_state: ResMut<NextState<OptionsState>>,
) {
    for app_state_edit in read_edit_app_state.read() {
        // search the vector associated with the current AppState in AppStateAllowedChanges struct
        //  if that vector contains the desired new AppState, then such state change is valid
        if (match current_app_state.get() {
            AppState::Splash => &allowed_changes.splash,
            AppState::Title => &allowed_changes.title,
            AppState::Options => &allowed_changes.options,
            AppState::Credits => &allowed_changes.credits,
            AppState::LoadSave => &allowed_changes.loadsave,
            AppState::Game => &allowed_changes.game,
            AppState::Fail => &allowed_changes.fail,
        })
        .contains(&app_state_edit.desired_app_state)
        {
            if app_state_edit.desired_app_state == AppState::Options {
                next_options_state.set(OptionsState::Menu);
            };
            next_app_state.set(app_state_edit.desired_app_state);
            info!(
                "AppState change: {:?} -> {:?}",
                current_app_state.get(),
                app_state_edit.desired_app_state
            );
        }
    }
}
