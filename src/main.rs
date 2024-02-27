mod config;
mod game;
mod graphics;
pub mod scene_directory;
mod states;
pub mod style;
mod ui;

use config::config_manager;
use game::game_manager;
use graphics::graphics_manager;
use states::state_manager;
use ui::ui_manager;

use bevy::{log::LogPlugin, prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use std::error::Error;

// MAIN ------------------------------------------------------------------------
// -----------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        // bevy plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    #[cfg(debug_assertions)]
                    level: bevy::log::Level::DEBUG,
                    #[cfg(not(debug_assertions))]
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        // non-bevy external plugins
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            WorldInspectorPlugin::new(),
        ))
        // my plugins
        .add_plugins((
            config_manager::ConfigManagerPlugin,
            game_manager::GameManagerPlugin,
            graphics_manager::GraphicsManagerPlugin,
            state_manager::StateManagerPlugin,
            ui_manager::UiManagerPlugin,
        ))
        .run();

    Ok(())
}
