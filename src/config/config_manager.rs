use super::{controls, windows};

use bevy::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct ConfigManagerPlugin;

impl Plugin for ConfigManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((controls::MyControlsPlugin, windows::WindowsPlugin));
    }
}
