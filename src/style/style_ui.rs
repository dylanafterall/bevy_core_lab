#![allow(unused)]

use super::{style_colors, style_fonts};

use bevy::prelude::*;

// CLEAR COLORS ----------------------------------------------------------------
// -----------------------------------------------------------------------------
pub const NORMAL_CLEAR_COLOR: Color = style_colors::LATTE_MAROON;

// UI COLORS -------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub const NORMAL_BUTTON_COLOR: Color = style_colors::LATTE_SKY;
pub const FOCUSED_BUTTON_COLOR: Color = style_colors::LATTE_TEAL;
pub const PRESSED_BUTTON_COLOR: Color = style_colors::LATTE_GREEN;

pub const NORMAL_BORDER_COLOR: Color = style_colors::LATTE_SUBTEXT0;
pub const FOCUSED_BORDER_COLOR: Color = style_colors::LATTE_OVERLAY0;
pub const PRESSED_BORDER_COLOR: Color = style_colors::LATTE_SURFACE0;

pub const NORMAL_TEXT_COLOR: Color = style_colors::LATTE_TEXT;
pub const FOCUSED_TEXT_COLOR: Color = style_colors::LATTE_RED;
pub const PRESSED_TEXT_COLOR: Color = style_colors::LATTE_GREEN;

// UI SIZES --------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub const NORMAL_TEXT_SIZE: f32 = 60.0;
pub const FOCUSED_TEXT_SIZE: f32 = 100.0;

// UI MATERIAL SHADERS ---------------------------------------------------------
// -----------------------------------------------------------------------------
pub const SHADER_COLOR: Color = Color::NONE;
pub const SHADER_BUTTON_TEXTURE: &str = "images/white.png";

// UI ENTITY SPAWN HELPERS -----------------------------------------------------
// -----------------------------------------------------------------------------
pub fn node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(300.0),
        height: Val::Px(130.0),
        border: UiRect::all(Val::Px(10.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center, // horizontally center
        align_items: AlignItems::Center,         // vertically center
        ..default()
    }
}

pub fn text_bundle(asset_server: &Res<AssetServer>, text: String) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load(style_fonts::FONT_BODY),
            font_size: NORMAL_TEXT_SIZE,
            color: NORMAL_TEXT_COLOR,
        },
    )
}
