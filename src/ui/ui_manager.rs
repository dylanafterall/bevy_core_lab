use super::{
    credits::*,
    gameover::*,
    loadsave::*,
    options::{audio::*, controls::*, general::*, menu::*, video::*},
    splash::*,
    title::*,
};
use crate::{
    config::controls,
    graphics::materials::materials_ui::{FocusedButtonMaterial, NormalButtonMaterial},
    states::app_state::AppState,
    style::{style_fonts, style_ui},
};

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct UiManagerPlugin;

impl Plugin for UiManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InitialUiFocus>()
            .add_plugins((
                OptionsMenuUIPlugin,
                AudioOptionsUIPlugin,
                ControlsOptionsUIPlugin,
                VideoOptionsUIPlugin,
                GeneralOptionsUIPlugin,
                SplashScreenUIPlugin,
                TitleScreenUIPlugin,
                LoadSaveScreenUIPlugin,
                CreditsScreenUIPlugin,
                GameoverScreenUIPlugin,
            ))
            .add_systems(
                Update,
                (
                    animate_focused_material.run_if(not(in_state(AppState::Game))),
                    style_ui_element.run_if(not(in_state(AppState::Game))),
                    handle_ui_navigation.run_if(not(in_state(AppState::Game))),
                ),
            );
    }
}

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Resource, Debug)]
pub struct InitialUiFocus {
    pub title: TitleElement,
    pub loadsave: LoadSaveElement,
    pub credits: CreditsElement,
    pub gameover: GameoverElement,
    pub options_menu: OptionsMenuElement,
    pub general_options: GeneralOptionsElement,
    pub controls_options: ControlsOptionsElement,
    pub audio_options: AudioOptionsElement,
    pub video_options: VideoOptionsElement,
}

impl Default for InitialUiFocus {
    fn default() -> InitialUiFocus {
        InitialUiFocus {
            title: TitleElement::Play,
            loadsave: LoadSaveElement::Profile1,
            credits: CreditsElement::Back,
            gameover: GameoverElement::Game,
            options_menu: OptionsMenuElement::General,
            general_options: GeneralOptionsElement::Back,
            controls_options: ControlsOptionsElement::Back,
            audio_options: AudioOptionsElement::Back,
            video_options: VideoOptionsElement::Back,
        }
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct UiFocusable {
    pub is_focused: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum UiElement {
    None,
    Title(TitleElement),
    LoadSave(LoadSaveElement),
    Gameover(GameoverElement),
    Credits(CreditsElement),
    OptionsMenu(OptionsMenuElement),
    GeneralOptions(GeneralOptionsElement),
    ControlsOptions(ControlsOptionsElement),
    AudioOptions(AudioOptionsElement),
    VideoOptions(VideoOptionsElement),
}

#[derive(Component, Clone, Copy, Debug)]
pub struct UiNavigation {
    pub self_id: UiElement,
    pub up: UiElement,
    pub down: UiElement,
    pub left: UiElement,
    pub right: UiElement,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn animate_focused_material(
    time: Res<Time>,
    mut ui_materials: ResMut<Assets<FocusedButtonMaterial>>,
) {
    for (_, material) in ui_materials.iter_mut() {
        material.time = time.elapsed_seconds();
    }
}

pub fn style_ui_element(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut normal_button_mat: ResMut<Assets<NormalButtonMaterial>>,
    mut focused_button_mat: ResMut<Assets<FocusedButtonMaterial>>,
    mut focus_query: Query<(Entity, &UiFocusable, &Children), Changed<UiFocusable>>,
    mut text_query: Query<&mut Text>,
) {
    let texture_handle: Handle<Image> = asset_server.load(style_ui::SHADER_BUTTON_TEXTURE);

    for (ui_entity, focusable, children) in &mut focus_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        if focusable.is_focused {
            text.sections[0].style.color = style_ui::FOCUSED_TEXT_COLOR;
            text.sections[0].style.font = asset_server.load(style_fonts::FONT_BODY_BOLD);
            text.sections[0].style.font_size = style_ui::FOCUSED_TEXT_SIZE;
            commands
                .entity(ui_entity)
                .remove::<Handle<NormalButtonMaterial>>()
                .insert(focused_button_mat.add(FocusedButtonMaterial {
                    time: time.elapsed_seconds(),
                    color_texture: Some(texture_handle.clone()),
                    alpha_mode: AlphaMode::Blend,
                }));
            debug!("UiFocusable change: set focus on entity {:?}", ui_entity);
        } else {
            text.sections[0].style.color = style_ui::NORMAL_TEXT_COLOR;
            text.sections[0].style.font = asset_server.load(style_fonts::FONT_BODY);
            text.sections[0].style.font_size = style_ui::NORMAL_TEXT_SIZE;
            commands
                .entity(ui_entity)
                .remove::<Handle<FocusedButtonMaterial>>()
                .insert(normal_button_mat.add(NormalButtonMaterial {
                    color_texture: Some(texture_handle.clone()),
                    alpha_mode: AlphaMode::Blend,
                }));
            debug!(
                "UiFocusable change: removed focus on entity {:?}",
                ui_entity
            );
        }
    }
}

pub fn handle_ui_navigation(
    action_state: Res<ActionState<controls::InputAction>>,
    mut ui_element_query: Query<(&UiNavigation, &mut UiFocusable)>,
) {
    if action_state.just_pressed(&controls::InputAction::Move) {
        let input_direction = action_state
            .clamped_axis_pair(&controls::InputAction::Move)
            .unwrap()
            .xy();

        // find the next ui_element to focus and un-set the current focused ui_element
        let mut next_ui_element = UiElement::None;
        for (ui_navigation, mut ui_focusable) in &mut ui_element_query {
            if ui_focusable.is_focused {
                if input_direction.y > 0.85 {
                    next_ui_element = ui_navigation.up;
                } else if input_direction.y < -0.85 {
                    next_ui_element = ui_navigation.down;
                } else if input_direction.x < -0.85 {
                    next_ui_element = ui_navigation.left;
                } else if input_direction.x > 0.85 {
                    next_ui_element = ui_navigation.right;
                }

                // if the next ui element to focus is valid, un-set the current ui element
                if next_ui_element != UiElement::None {
                    ui_focusable.is_focused = false;
                }

                break;
            }
        }

        // set focus on the next ui_element
        if next_ui_element != UiElement::None {
            for (ui_navigation, mut ui_focusable) in &mut ui_element_query {
                if ui_navigation.self_id == next_ui_element {
                    ui_focusable.is_focused = true;

                    break;
                }
            }
        }
    }
}
