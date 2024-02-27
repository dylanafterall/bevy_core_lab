#![allow(unused)]

use crate::graphics::materials::materials_transition::*;

use bevy::prelude::*;

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub enum TransitionOption {
    FadeToColor(Vec4),
    FadeFromColor(Vec4),
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct Transition {
    pub desired_transition: TransitionOption,
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct TransitionNode {
    pub timer: Timer,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_transition(
    mut commands: Commands,
    mut read_transition_event: EventReader<Transition>,
    mut fade_to_color_mat: ResMut<Assets<FadeToColorMaterial>>,
    mut fade_from_color_mat: ResMut<Assets<FadeFromColorMaterial>>,
) {
    for transition_event in read_transition_event.read() {
        let transition = commands
            .spawn((
                Name::new("TransitionNode"),
                TransitionNode {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                },
                MaterialNodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    material: fade_to_color_mat.add(FadeToColorMaterial {
                        time: 0.0,
                        color: Vec4::new(0.0, 0.0, 0.0, 1.0), // default BLACK
                    }),
                    ..default()
                },
            ))
            .id();

        match transition_event.desired_transition {
            TransitionOption::FadeToColor(desired_color) => {
                commands
                    .entity(transition)
                    .remove::<Handle<FadeToColorMaterial>>()
                    .insert(fade_to_color_mat.add(FadeToColorMaterial {
                        time: 0.0,
                        color: desired_color,
                    }));
            }
            TransitionOption::FadeFromColor(desired_color) => {
                commands
                    .entity(transition)
                    .remove::<Handle<FadeToColorMaterial>>()
                    .insert(fade_from_color_mat.add(FadeFromColorMaterial {
                        time: 1.0,
                        color: desired_color,
                    }));
            }
        }
    }
}

pub fn animate_transition(
    mut commands: Commands,
    mut transition_node_query: Query<(Entity, &mut TransitionNode)>,
    time: Res<Time>,
    mut fade_to_color_materials: ResMut<Assets<FadeToColorMaterial>>,
    mut fade_from_color_materials: ResMut<Assets<FadeFromColorMaterial>>,
) {
    for (transition_entity, mut transition_node) in transition_node_query.iter_mut() {
        transition_node.timer.tick(time.delta());

        // fade to color
        for (_, material) in fade_to_color_materials.iter_mut() {
            material.time = transition_node.timer.elapsed_secs(); // time goes 0.0 -> 1.0
        }

        // fade from color
        for (_, material) in fade_from_color_materials.iter_mut() {
            material.time = transition_node.timer.remaining_secs(); // time goes 1.0 -> 0.0
        }

        if transition_node.timer.just_finished() {
            commands.entity(transition_entity).despawn_recursive();
        }
    }
}
