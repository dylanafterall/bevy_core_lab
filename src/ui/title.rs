use super::ui_manager::{InitialUiFocus, UiElement, UiFocusable, UiNavigation};
use crate::{
    config::controls,
    graphics::materials::materials_ui::NormalButtonMaterial,
    states::app_state::{AppState, EditAppState},
    style::style_ui,
};

use bevy::{app::AppExit, prelude::*};
use leafwing_input_manager::action_state::ActionState;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct TitleScreenUIPlugin;

impl Plugin for TitleScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Title), spawn_title)
            .add_systems(OnExit(AppState::Title), despawn_title)
            .add_systems(
                Update,
                handle_ui_selection.run_if(in_state(AppState::Title)),
            );
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct TitleNode;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum TitleElement {
    Play,
    Options,
    Credits,
    Quit,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_focus: Res<InitialUiFocus>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
) {
    let button_texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    let node = commands
        .spawn((Name::new("TitleNode"), TitleNode, style_ui::node_bundle()))
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
    let button_play = button("Play".to_string());
    let button_options = button("Options".to_string());
    let button_credits = button("Credits".to_string());
    let button_quit = button("Quit".to_string());

    // set ui navigation for all elements
    commands.entity(button_play).insert(UiNavigation {
        self_id: UiElement::Title(TitleElement::Play),
        up: UiElement::Title(TitleElement::Quit),
        down: UiElement::Title(TitleElement::Options),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_options).insert(UiNavigation {
        self_id: UiElement::Title(TitleElement::Options),
        up: UiElement::Title(TitleElement::Play),
        down: UiElement::Title(TitleElement::Credits),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_credits).insert(UiNavigation {
        self_id: UiElement::Title(TitleElement::Credits),
        up: UiElement::Title(TitleElement::Options),
        down: UiElement::Title(TitleElement::Quit),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_quit).insert(UiNavigation {
        self_id: UiElement::Title(TitleElement::Quit),
        up: UiElement::Title(TitleElement::Credits),
        down: UiElement::Title(TitleElement::Play),
        left: UiElement::None,
        right: UiElement::None,
    });

    // set the initial focused entity when title screen spawns
    let focus: Entity = match initial_focus.title {
        TitleElement::Play => button_play,
        TitleElement::Options => button_options,
        TitleElement::Credits => button_credits,
        TitleElement::Quit => button_quit,
    };
    commands
        .entity(focus)
        .remove::<UiFocusable>()
        .insert(UiFocusable { is_focused: true });

    // make the buttons children of the parent node
    commands.entity(node).push_children(&[button_play]);
    commands.entity(node).push_children(&[button_options]);
    commands.entity(node).push_children(&[button_credits]);
    commands.entity(node).push_children(&[button_quit]);
}

pub fn despawn_title(mut commands: Commands, menu_query: Query<Entity, With<TitleNode>>) {
    for title_menu_entity in menu_query.iter() {
        commands.entity(title_menu_entity).despawn_recursive();
    }
}

pub fn handle_ui_selection(
    action_state: Res<ActionState<controls::InputAction>>,
    mut initial_focus: ResMut<InitialUiFocus>,
    mut ui_element_query: Query<(&UiNavigation, &mut UiFocusable)>,
    mut write_edit_app_state: EventWriter<EditAppState>,
    mut write_app_exit: EventWriter<AppExit>,
) {
    if action_state.just_pressed(&controls::InputAction::Select) {
        for (ui_navigation, ui_focusable) in &mut ui_element_query {
            if ui_focusable.is_focused {
                match ui_navigation.self_id {
                    UiElement::Title(title_element) => match title_element {
                        TitleElement::Play => {
                            initial_focus.title = TitleElement::Play;
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::LoadSave,
                            });
                        }
                        TitleElement::Options => {
                            initial_focus.title = TitleElement::Options;
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Options,
                            });
                        }
                        TitleElement::Credits => {
                            initial_focus.title = TitleElement::Credits;
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Credits,
                            });
                        }
                        TitleElement::Quit => {
                            write_app_exit.send(AppExit);
                        }
                    },
                    _ => {
                        error!(
                            "UiElement {:?} encountered non-UiElement::Title entity on title screen",
                            ui_navigation
                        );
                    }
                }

                break;
            }
        }
    }
}
