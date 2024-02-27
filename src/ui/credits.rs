use super::ui_manager::{InitialUiFocus, UiElement, UiFocusable, UiNavigation};
use crate::{
    config::controls,
    graphics::materials::materials_ui::NormalButtonMaterial,
    states::app_state::{AppState, EditAppState},
    style::style_ui,
};

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct CreditsScreenUIPlugin;

impl Plugin for CreditsScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Credits), spawn_credits)
            .add_systems(OnExit(AppState::Credits), despawn_credits)
            .add_systems(
                Update,
                handle_ui_selection.run_if(in_state(AppState::Credits)),
            );
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct CreditsNode;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum CreditsElement {
    Back,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_credits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_focus: Res<InitialUiFocus>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
) {
    let button_texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    let node = commands
        .spawn((
            Name::new("CreditsNode"),
            CreditsNode,
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
        self_id: UiElement::Credits(CreditsElement::Back),
        up: UiElement::None,
        down: UiElement::None,
        left: UiElement::None,
        right: UiElement::None,
    });

    // set the initial focused entity when title screen spawns
    let focus: Entity = match initial_focus.credits {
        CreditsElement::Back => button_back,
    };
    commands
        .entity(focus)
        .remove::<UiFocusable>()
        .insert(UiFocusable { is_focused: true });

    // make the buttons children of the parent node
    commands.entity(node).push_children(&[button_back]);
}

pub fn despawn_credits(mut commands: Commands, menu_query: Query<Entity, With<CreditsNode>>) {
    for credits_entity in menu_query.iter() {
        commands.entity(credits_entity).despawn_recursive();
    }
}

pub fn handle_ui_selection(
    action_state: Res<ActionState<controls::InputAction>>,
    mut initial_focus: ResMut<InitialUiFocus>,
    mut ui_element_query: Query<(&UiNavigation, &mut UiFocusable)>,
    mut write_edit_app_state: EventWriter<EditAppState>,
) {
    if action_state.just_pressed(&controls::InputAction::Select) {
        for (ui_navigation, ui_focusable) in &mut ui_element_query {
            if ui_focusable.is_focused {
                match ui_navigation.self_id {
                    UiElement::Credits(credits_element) => match credits_element {
                        CreditsElement::Back => {
                            initial_focus.credits = CreditsElement::Back;
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Title,
                            });
                        }
                    },
                    _ => {
                        error!(
                            "UiElement {:?} encountered non-UiElement::Credits entity on credits screen",
                            ui_navigation
                        );
                    }
                }

                break;
            }
        }
    }
}
