use super::{
    materials::{materials_transition::*, materials_ui::*},
    transition::*,
};

use bevy::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct GraphicsManagerPlugin;

impl Plugin for GraphicsManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiMaterialPlugin::<NormalButtonMaterial>::default(),
            UiMaterialPlugin::<FocusedButtonMaterial>::default(),
            UiMaterialPlugin::<FadeToColorMaterial>::default(),
            UiMaterialPlugin::<FadeFromColorMaterial>::default(),
        ))
        .add_event::<Transition>()
        .add_systems(Update, (spawn_transition, animate_transition));
    }
}
