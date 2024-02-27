use crate::{
    states::app_state::{AppState, EditAppState},
    style::style_splash,
};

use bevy::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct SplashScreenUIPlugin;

impl Plugin for SplashScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), spawn_splash)
            .add_systems(OnExit(AppState::Splash), despawn_splash)
            .add_systems(Update, splash_animations.run_if(in_state(AppState::Splash)));
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct SplashNode {
    pub splash_timer: Timer,
    pub studio_logo_timer: Timer,
    pub engine_logo_timer: Timer,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("SplashNode"),
            SplashNode {
                splash_timer: Timer::from_seconds(6.0, TimerMode::Once),
                studio_logo_timer: Timer::from_seconds(1.5, TimerMode::Once),
                engine_logo_timer: Timer::from_seconds(1.5, TimerMode::Once),
            },
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((ImageBundle {
                image: UiImage::new(asset_server.load(style_splash::STUDIO_LOGO_TEXTURE)),
                ..default()
            },));
        });
}

pub fn despawn_splash(mut commands: Commands, splash_query: Query<Entity, With<SplashNode>>) {
    for splash_node in splash_query.iter() {
        commands.entity(splash_node).despawn_recursive();
    }
}

pub fn splash_animations(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut node_query: Query<(&mut SplashNode, &mut Visibility, &Children)>,
    mut logo_query: Query<&mut UiImage>,
    mut write_edit_app_state: EventWriter<EditAppState>,
) {
    let (mut splash_node, mut node_visibility, children) = node_query.single_mut();
    let mut logo_image = logo_query.get_mut(children[0]).unwrap();

    splash_node.splash_timer.tick(time.delta());

    // 0.0 : start splash screen
    // 1.0 : display studio logo
    // 2.5 : hide studio logo
    // 3.5 : display engine logo
    // 5.0 : hide engine logo
    // 6.0 : end splash screen
    if splash_node.splash_timer.elapsed_secs() >= 6.0 {
        write_edit_app_state.send(EditAppState {
            desired_app_state: AppState::Title,
        });
    } else if splash_node.splash_timer.elapsed_secs() >= 3.5 {
        if !splash_node.engine_logo_timer.finished() {
            *node_visibility = Visibility::Visible;
        }
        splash_node.engine_logo_timer.tick(time.delta());
    } else if splash_node.splash_timer.elapsed_secs() >= 1.0 {
        if !splash_node.studio_logo_timer.finished() {
            *node_visibility = Visibility::Visible;
        }
        splash_node.studio_logo_timer.tick(time.delta());
    }

    if splash_node.engine_logo_timer.just_finished() {
        *node_visibility = Visibility::Hidden;
    }
    if splash_node.studio_logo_timer.just_finished() {
        *node_visibility = Visibility::Hidden;
        *logo_image = UiImage::new(asset_server.load(style_splash::ENGINE_LOGO_TEXTURE));
    }
}
