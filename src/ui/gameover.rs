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
pub struct GameoverScreenUIPlugin;

impl Plugin for GameoverScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Fail), spawn_gameover)
            .add_systems(OnExit(AppState::Fail), despawn_gameover)
            .add_systems(Update, handle_ui_selection.run_if(in_state(AppState::Fail)));
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct GameoverNode;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum GameoverElement {
    Game,
    Title,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_focus: Res<InitialUiFocus>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
) {
    let button_texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    let node = commands
        .spawn((
            Name::new("GameoverNode"),
            GameoverNode,
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
    let button_game = button("Game".to_string());
    let button_title = button("Title".to_string());

    // set ui navigation for all elements
    commands.entity(button_game).insert(UiNavigation {
        self_id: UiElement::Gameover(GameoverElement::Game),
        up: UiElement::Gameover(GameoverElement::Title),
        down: UiElement::Gameover(GameoverElement::Title),
        left: UiElement::None,
        right: UiElement::None,
    });
    commands.entity(button_title).insert(UiNavigation {
        self_id: UiElement::Gameover(GameoverElement::Title),
        up: UiElement::Gameover(GameoverElement::Game),
        down: UiElement::Gameover(GameoverElement::Game),
        left: UiElement::None,
        right: UiElement::None,
    });

    // set the initial focused entity when title screen spawns
    let focus: Entity = match initial_focus.gameover {
        GameoverElement::Game => button_game,
        GameoverElement::Title => button_title,
    };
    commands
        .entity(focus)
        .remove::<UiFocusable>()
        .insert(UiFocusable { is_focused: true });

    // make the buttons children of the parent node
    commands.entity(node).push_children(&[button_game]);
    commands.entity(node).push_children(&[button_title]);
}

pub fn despawn_gameover(mut commands: Commands, menu_query: Query<Entity, With<GameoverNode>>) {
    for gameover_entity in menu_query.iter() {
        commands.entity(gameover_entity).despawn_recursive();
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
                    UiElement::Gameover(gameover_element) => match gameover_element {
                        GameoverElement::Game => {
                            initial_focus.gameover = GameoverElement::Game;
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Game,
                            });
                        }
                        GameoverElement::Title => {
                            initial_focus.gameover = GameoverElement::Game;
                            write_edit_app_state.send(EditAppState {
                                desired_app_state: AppState::Title,
                            });
                        }
                    },
                    _ => {
                        error!(
                            "UiElement {:?} encountered non-UiElement::Gameover entity on gameover screen",
                            ui_navigation
                        );
                    }
                }

                break;
            }
        }
    }
}
