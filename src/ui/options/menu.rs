use super::super::ui_manager::{InitialUiFocus, UiElement, UiFocusable, UiNavigation};
use crate::{
    config::controls,
    graphics::materials::materials_ui::NormalButtonMaterial,
    states::{
        app_state::{AppState, EditAppState},
        options_state::{EditOptionsState, OptionsState},
    },
    style::style_ui,
};

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct OptionsMenuUIPlugin;

impl Plugin for OptionsMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(OptionsState::Menu), spawn_options_menu)
            .add_systems(OnExit(AppState::Options), despawn_options_menu)
            .add_systems(OnExit(OptionsState::Menu), despawn_options_menu)
            .add_systems(
                Update,
                handle_ui_selection.run_if(in_state(OptionsState::Menu)),
            );
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct OptionsMenuNode;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum OptionsMenuElement {
    General,
    Controls,
    Audio,
    Video,
    Back,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_options_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_focus: Res<InitialUiFocus>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
) {
    let button_texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    let node = commands
        .spawn((
            Name::new("OptionsMenuNode"),
            OptionsMenuNode,
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
    let button_general = button("General".to_string());
    let button_controls = button("Controls".to_string());
    let button_audio = button("Audio".to_string());
    let button_video = button("Video".to_string());
    let button_back = button("Back".to_string());

    // set ui navigation for all elements
    commands.entity(button_general).insert(UiNavigation {
        self_id: UiElement::OptionsMenu(OptionsMenuElement::General),
        up: UiElement::OptionsMenu(OptionsMenuElement::Back),
        down: UiElement::OptionsMenu(OptionsMenuElement::Controls),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_controls).insert(UiNavigation {
        self_id: UiElement::OptionsMenu(OptionsMenuElement::Controls),
        up: UiElement::OptionsMenu(OptionsMenuElement::General),
        down: UiElement::OptionsMenu(OptionsMenuElement::Audio),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_audio).insert(UiNavigation {
        self_id: UiElement::OptionsMenu(OptionsMenuElement::Audio),
        up: UiElement::OptionsMenu(OptionsMenuElement::Controls),
        down: UiElement::OptionsMenu(OptionsMenuElement::Video),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_video).insert(UiNavigation {
        self_id: UiElement::OptionsMenu(OptionsMenuElement::Video),
        up: UiElement::OptionsMenu(OptionsMenuElement::Audio),
        down: UiElement::OptionsMenu(OptionsMenuElement::Back),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_back).insert(UiNavigation {
        self_id: UiElement::OptionsMenu(OptionsMenuElement::Back),
        up: UiElement::OptionsMenu(OptionsMenuElement::Video),
        down: UiElement::OptionsMenu(OptionsMenuElement::General),
        left: UiElement::None,
        right: UiElement::None,
    });

    // set the initial focused entity when title screen spawns
    let focus: Entity = match initial_focus.options_menu {
        OptionsMenuElement::General => button_general,
        OptionsMenuElement::Controls => button_controls,
        OptionsMenuElement::Audio => button_audio,
        OptionsMenuElement::Video => button_video,
        OptionsMenuElement::Back => button_back,
    };
    commands
        .entity(focus)
        .remove::<UiFocusable>()
        .insert(UiFocusable { is_focused: true });

    // make the buttons children of the parent node
    commands.entity(node).push_children(&[button_general]);
    commands.entity(node).push_children(&[button_controls]);
    commands.entity(node).push_children(&[button_audio]);
    commands.entity(node).push_children(&[button_video]);
    commands.entity(node).push_children(&[button_back]);
}

pub fn despawn_options_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<OptionsMenuNode>>,
) {
    for options_entity in menu_query.iter() {
        commands.entity(options_entity).despawn_recursive();
    }
}

pub fn handle_ui_selection(
    action_state: Res<ActionState<controls::InputAction>>,
    mut initial_focus: ResMut<InitialUiFocus>,
    mut ui_element_query: Query<(&UiNavigation, &mut UiFocusable)>,
    mut write_edit_app_state: EventWriter<EditAppState>,
    mut write_edit_options_state: EventWriter<EditOptionsState>,
) {
    if action_state.just_pressed(&controls::InputAction::Select) {
        for (ui_navigation, ui_focusable) in &mut ui_element_query {
            if ui_focusable.is_focused {
                match ui_navigation.self_id {
                    UiElement::OptionsMenu(options_menu_element) => {
                        match options_menu_element {
                            OptionsMenuElement::General => {
                                initial_focus.options_menu = OptionsMenuElement::General;
                                write_edit_options_state.send(EditOptionsState {
                                    desired_options_state: OptionsState::General,
                                });
                            }
                            OptionsMenuElement::Controls => {
                                initial_focus.options_menu = OptionsMenuElement::Controls;
                                write_edit_options_state.send(EditOptionsState {
                                    desired_options_state: OptionsState::Controls,
                                });
                            }
                            OptionsMenuElement::Audio => {
                                initial_focus.options_menu = OptionsMenuElement::Audio;
                                write_edit_options_state.send(EditOptionsState {
                                    desired_options_state: OptionsState::Audio,
                                });
                            }
                            OptionsMenuElement::Video => {
                                initial_focus.options_menu = OptionsMenuElement::Video;
                                write_edit_options_state.send(EditOptionsState {
                                    desired_options_state: OptionsState::Video,
                                });
                            }
                            OptionsMenuElement::Back => {
                                initial_focus.options_menu = OptionsMenuElement::General;
                                // switch OptionsState to avoid scheduling conflicts w/ AppState::Options
                                write_edit_options_state.send(EditOptionsState {
                                    desired_options_state: OptionsState::None,
                                });
                                write_edit_app_state.send(EditAppState {
                                    desired_app_state: AppState::Title,
                                });
                            }
                        }
                    }
                    _ => {
                        error!(
                            "UiElement {:?} encountered non-UiElement::OptionsMenu entity on options menu screen",
                            ui_navigation
                        );
                    }
                }

                break;
            }
        }
    }
}
