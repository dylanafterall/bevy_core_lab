use super::super::ui_manager::{InitialUiFocus, UiElement, UiFocusable, UiNavigation};
use crate::{
    config::controls,
    graphics::materials::materials_ui::NormalButtonMaterial,
    states::options_state::{EditOptionsState, OptionsState},
    style::style_ui,
};

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct GeneralOptionsUIPlugin;

impl Plugin for GeneralOptionsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OptionsState::General), spawn_gameplay_options)
            .add_systems(OnExit(OptionsState::General), despawn_gameplay_options)
            .add_systems(
                Update,
                handle_ui_selection.run_if(in_state(OptionsState::General)),
            );
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct GeneralOptionsNode;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum GeneralOptionsElement {
    Back,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_gameplay_options(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_focus: Res<InitialUiFocus>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
) {
    let button_texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    let node = commands
        .spawn((
            Name::new("GeneralOptionsNode"),
            GeneralOptionsNode,
            style_ui::node_bundle(),
        ))
        .id();

    let mut button = |text: String| {
        commands
            .spawn((
                UiFocusable { is_focused: false },
                MaterialNodeBundle {
                    style: style_ui::button_style(),
                    material: normal_button_mat.add(NormalButtonMaterial {
                        color_texture: Some(button_texture_handle.clone()),
                        alpha_mode: AlphaMode::Blend,
                    }),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(style_ui::text_bundle(&asset_server, text));
            })
            .id()
    };

    // create the buttons
    let button_back = button("Back".to_string());

    // set ui navigation for all elements
    commands.entity(button_back).insert(UiNavigation {
        self_id: UiElement::GeneralOptions(GeneralOptionsElement::Back),
        up: UiElement::None,
        down: UiElement::None,
        left: UiElement::None,
        right: UiElement::None,
    });

    // set the initial focused entity when title screen spawns
    let focus: Entity = match initial_focus.general_options {
        GeneralOptionsElement::Back => button_back,
    };
    commands
        .entity(focus)
        .remove::<UiFocusable>()
        .insert(UiFocusable { is_focused: true });

    // make the buttons children of the parent node
    commands.entity(node).push_children(&[button_back]);
}

pub fn despawn_gameplay_options(
    mut commands: Commands,
    menu_query: Query<Entity, With<GeneralOptionsNode>>,
) {
    for gameplay_entity in menu_query.iter() {
        commands.entity(gameplay_entity).despawn_recursive();
    }
}

pub fn handle_ui_selection(
    action_state: Res<ActionState<controls::InputAction>>,
    mut initial_focus: ResMut<InitialUiFocus>,
    mut ui_element_query: Query<(&UiNavigation, &mut UiFocusable)>,
    mut write_edit_options_state: EventWriter<EditOptionsState>,
) {
    if action_state.just_pressed(&controls::InputAction::Select) {
        for (ui_navigation, ui_focusable) in &mut ui_element_query {
            if ui_focusable.is_focused {
                match ui_navigation.self_id {
                    UiElement::GeneralOptions(general_element) => match general_element {
                        GeneralOptionsElement::Back => {
                            initial_focus.general_options = GeneralOptionsElement::Back;
                            write_edit_options_state.send(EditOptionsState {
                                desired_options_state: OptionsState::Menu,
                            });
                        }
                    },
                    _ => {
                        error!(
                            "UiElement {:?} encountered non-UiElement::GeneralOptions entity on general options screen",
                            ui_navigation
                        );
                    }
                }

                break;
            }
        }
    }
}
