use super::ui_manager::{InitialUiFocus, UiElement, UiFocusable, UiNavigation};
use crate::{
    config::controls,
    graphics::materials::materials_ui::NormalButtonMaterial,
    states::{
        app_state::{AppState, EditAppState},
        profile_state::{EditProfileState, ProfileState},
    },
    style::style_ui,
};

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct LoadSaveScreenUIPlugin;

impl Plugin for LoadSaveScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadSave), spawn_loadsave)
            .add_systems(OnExit(AppState::LoadSave), despawn_loadsave)
            .add_systems(
                Update,
                handle_ui_selection.run_if(in_state(AppState::LoadSave)),
            );
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct LoadSaveNode;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum LoadSaveElement {
    Profile1,
    Profile2,
    Profile3,
    Profile4,
    Back,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_loadsave(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_focus: Res<InitialUiFocus>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
) {
    let button_texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    let node = commands
        .spawn((
            Name::new("LoadSaveNode"),
            LoadSaveNode,
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
    let button_profile_1 = button("Profile 1".to_string());
    let button_profile_2 = button("Profile 2".to_string());
    let button_profile_3 = button("Profile 3".to_string());
    let button_profile_4 = button("Profile 4".to_string());
    let button_back = button("Back".to_string());

    // set ui navigation for all elements
    commands.entity(button_profile_1).insert(UiNavigation {
        self_id: UiElement::LoadSave(LoadSaveElement::Profile1),
        up: UiElement::LoadSave(LoadSaveElement::Back),
        down: UiElement::LoadSave(LoadSaveElement::Profile2),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_profile_2).insert(UiNavigation {
        self_id: UiElement::LoadSave(LoadSaveElement::Profile2),
        up: UiElement::LoadSave(LoadSaveElement::Profile1),
        down: UiElement::LoadSave(LoadSaveElement::Profile3),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_profile_3).insert(UiNavigation {
        self_id: UiElement::LoadSave(LoadSaveElement::Profile3),
        up: UiElement::LoadSave(LoadSaveElement::Profile2),
        down: UiElement::LoadSave(LoadSaveElement::Profile4),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_profile_4).insert(UiNavigation {
        self_id: UiElement::LoadSave(LoadSaveElement::Profile4),
        up: UiElement::LoadSave(LoadSaveElement::Profile3),
        down: UiElement::LoadSave(LoadSaveElement::Back),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_back).insert(UiNavigation {
        self_id: UiElement::LoadSave(LoadSaveElement::Back),
        up: UiElement::LoadSave(LoadSaveElement::Profile4),
        down: UiElement::LoadSave(LoadSaveElement::Profile1),
        left: UiElement::None,
        right: UiElement::None,
    });

    // set the initial focused entity when title screen spawns
    let focus: Entity = match initial_focus.loadsave {
        LoadSaveElement::Profile1 => button_profile_1,
        LoadSaveElement::Profile2 => button_profile_2,
        LoadSaveElement::Profile3 => button_profile_3,
        LoadSaveElement::Profile4 => button_profile_4,
        LoadSaveElement::Back => button_back,
    };
    commands
        .entity(focus)
        .remove::<UiFocusable>()
        .insert(UiFocusable { is_focused: true });

    // make the buttons children of the parent node
    commands.entity(node).push_children(&[button_profile_1]);
    commands.entity(node).push_children(&[button_profile_2]);
    commands.entity(node).push_children(&[button_profile_3]);
    commands.entity(node).push_children(&[button_profile_4]);
    commands.entity(node).push_children(&[button_back]);
}

pub fn despawn_loadsave(mut commands: Commands, menu_query: Query<Entity, With<LoadSaveNode>>) {
    for loadsave_entity in menu_query.iter() {
        commands.entity(loadsave_entity).despawn_recursive();
    }
}

pub fn handle_ui_selection(
    action_state: Res<ActionState<controls::InputAction>>,
    mut initial_focus: ResMut<InitialUiFocus>,
    mut ui_element_query: Query<(&UiNavigation, &mut UiFocusable)>,
    mut write_edit_app_state: EventWriter<EditAppState>,
    mut write_edit_profile_state: EventWriter<EditProfileState>,
) {
    if action_state.just_pressed(&controls::InputAction::Select) {
        for (ui_navigation, ui_focusable) in &mut ui_element_query {
            if ui_focusable.is_focused {
                match ui_navigation.self_id {
                    UiElement::LoadSave(loadsave_element) => match loadsave_element {
                        LoadSaveElement::Profile1 => {
                            initial_focus.loadsave = LoadSaveElement::Profile1;
                            write_edit_profile_state.send(EditProfileState {
                                desired_profile_state: ProfileState::Profile1,
                            });
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Game,
                            });
                        }
                        LoadSaveElement::Profile2 => {
                            initial_focus.loadsave = LoadSaveElement::Profile2;
                            write_edit_profile_state.send(EditProfileState {
                                desired_profile_state: ProfileState::Profile2,
                            });
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Game,
                            });
                        }
                        LoadSaveElement::Profile3 => {
                            initial_focus.loadsave = LoadSaveElement::Profile3;
                            write_edit_profile_state.send(EditProfileState {
                                desired_profile_state: ProfileState::Profile3,
                            });
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Game,
                            });
                        }
                        LoadSaveElement::Profile4 => {
                            initial_focus.loadsave = LoadSaveElement::Profile4;
                            write_edit_profile_state.send(EditProfileState {
                                desired_profile_state: ProfileState::Profile4,
                            });
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Game,
                            });
                        }
                        LoadSaveElement::Back => {
                            initial_focus.loadsave = LoadSaveElement::Profile1; // reset
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Title,
                            });
                        }
                    },
                    _ => {
                        error!(
                            "UiElement {:?} encountered non-UiElement::LoadSave entity on loadsave screen",
                            ui_navigation
                        );
                    }
                }

                break;
            }
        }
    }
}
