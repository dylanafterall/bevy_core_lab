/* -----------------------------------------------------------------------------
    Editing window resolution also edits cameras via handle_resolution_edit()
        for 2D cameras, adjust OrthographicProjection{scaling_mode: Fixed{width: and height:}}
            (w, h) [ 4:3 ]  = (16, 12)  = (256, 192)
            (w, h) [ 5:4 ]  = (15, 12)  = (240, 192)
            (w, h) [ 8:5 ]  = (16, 10)  = (256, 160)
            (w, h) [ 16:9 ] = (16, 9)   = (256, 144)
            (w, h) [ 21:9 ] = (21, 9)   = (336, 144)
        for 3D cameras, adjust transform: Transform::from_translation(_,_,z)
            FOV = π/4 radians or 45 degrees (bevy engine default)
            z = ( height / 2 ) / ( tan( FOV / 2 ) )
            z [ 4:3 ]   = 231.765
            z [ 5:4 ]   = 231.765
            z [ 8:5 ]   = 193.137
            z [ 16:9 ]  = 173.823
            z [ 21:9 ]  = 173.823

      ORDER       RENDER LAYER            CAMERA              CONFIG
      -----       ------------            ------              ------
        0             1               Background          3D Perspective
        1             0               Occlusion, Debug    2D Orthographic
        2             2               UI                  2D Orthographic
----------------------------------------------------------------------------- */
use crate::config::windows::{AspectRatio, EditResolution};
use crate::states::app_state::AppState;

use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::{Projection::Perspective, *},
    render::{
        camera::{CameraOutputMode, ScalingMode},
        render_resource::{BlendState, LoadOp},
        view::RenderLayers,
    },
};

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraTranslation>()
            .add_event::<CameraZoom>()
            .add_systems(Startup, spawn_cameras)
            .add_systems(
                Update,
                (
                    handle_edit_resolution.run_if(in_state(AppState::Options)),
                    handle_translate_camera.run_if(in_state(AppState::Game)),
                    handle_zoom_camera.run_if(in_state(AppState::Game)),
                ),
            );
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct StageCamera;

#[derive(Component)]
pub struct UICamera;

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------

#[derive(Event)]
pub struct CameraTranslation {
    pub position: Vec2,
}

#[derive(Event)]
pub struct CameraZoom {
    is_directed_in: bool,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_cameras(mut commands: Commands) {
    // TODO: all cameras currently assume a 16:9 aspect ratio when spawning. replace with options resource / save file
    // MAIN
    // ----------
    commands.spawn((
        Name::new("CameraMain"),
        WorldCamera,
        RenderLayers::from_layers(&[1]),
        Camera3dBundle {
            camera: Camera {
                order: 0,
                hdr: true,
                ..default()
            },
            projection: Perspective(PerspectiveProjection { ..default() }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 173.82)),
            ..default()
        },
    ));

    // STAGE
    // -----
    commands.spawn((
        Name::new("CameraStage"),
        StageCamera,
        RenderLayers::from_layers(&[0]),
        Camera2dBundle {
            camera: Camera {
                order: 1,
                hdr: true,
                msaa_writeback: false,
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    color_attachment_load_op: LoadOp::Load,
                },
                ..default()
            },
            projection: OrthographicProjection {
                near: -1.0,
                scaling_mode: ScalingMode::Fixed {
                    width: 256.0,
                    height: 144.0,
                },
                ..default()
            },
            ..default()
        },
    ));

    // UI
    // --
    commands.spawn((
        Name::new("CameraUI"),
        UICamera,
        RenderLayers::from_layers(&[2]),
        Camera2dBundle {
            camera: Camera {
                order: 2,
                hdr: true,
                msaa_writeback: false,
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    color_attachment_load_op: LoadOp::Load,
                },
                ..default()
            },
            projection: OrthographicProjection {
                near: -1.0,
                scaling_mode: ScalingMode::Fixed {
                    width: 256.0,
                    height: 144.0,
                },
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.1,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 1.0,
                threshold_softness: 0.0,
            },
            ..default()
        },
    ));
}

pub fn handle_edit_resolution(
    mut orthographic_query: Query<&mut OrthographicProjection>,
    mut perspective_query: Query<&mut Transform, With<Projection>>,
    mut read_edit_resolution: EventReader<EditResolution>,
) {
    for res_edit in read_edit_resolution.read() {
        //  edit the Fixed Width and Height for orthographic projections
        //  edit the z position of cameras with perspective projections
        match &res_edit.aspect_ratio {
            AspectRatio::_4_3 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = ScalingMode::Fixed {
                        width: 256.0,
                        height: 192.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 231.765);
                }
            }
            AspectRatio::_5_4 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = ScalingMode::Fixed {
                        width: 240.0,
                        height: 192.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 231.765);
                }
            }
            AspectRatio::_8_5 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = ScalingMode::Fixed {
                        width: 256.0,
                        height: 160.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 193.137);
                }
            }
            AspectRatio::_16_9 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = ScalingMode::Fixed {
                        width: 256.0,
                        height: 144.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 173.823);
                }
            }
            AspectRatio::_21_9 => {
                for mut camera2d in orthographic_query.iter_mut() {
                    camera2d.scaling_mode = ScalingMode::Fixed {
                        width: 336.0,
                        height: 144.0,
                    };
                }
                for mut camera3d in perspective_query.iter_mut() {
                    let temp_translation = camera3d.translation;
                    camera3d.translation =
                        Vec3::new(temp_translation.x, temp_translation.y, 173.823);
                }
            }
        };
    }
}

pub fn handle_translate_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut read_translate_camera: EventReader<CameraTranslation>,
) {
    for cam_edit in read_translate_camera.read() {
        for mut camera in camera_query.iter_mut() {
            camera.translation = Vec3::new(
                cam_edit.position.x,
                cam_edit.position.y,
                camera.translation.z, // don't change the current depth (z) value
            );
        }
    }
}

pub fn handle_zoom_camera(
    mut orthographic_query: Query<&mut OrthographicProjection, Without<UICamera>>,
    mut perspective_query: Query<(&mut Projection, &Transform)>,
    mut read_zoom_camera: EventReader<CameraZoom>,
) {
    for cam_zoom in read_zoom_camera.read() {
        // will use aspect ratio and scale of 2d cameras to calculate new FOV for 3d cameras
        let mut aspect_ratio_y = 0.0;
        let mut new_scale = 0.0;

        for mut camera_2d in orthographic_query.iter_mut() {
            if cam_zoom.is_directed_in {
                camera_2d.scale += 0.25; // zoom camera IN
            } else {
                camera_2d.scale -= 0.25; // zoom camera OUT
            }
            camera_2d.scale = camera_2d.scale.clamp(0.25, 5.0);

            new_scale = camera_2d.scale;

            if let ScalingMode::Fixed { height, .. } = camera_2d.scaling_mode {
                aspect_ratio_y = height;
            }
        }

        // top = y coordinate of perspective projection frustum top as it intersects z=0 plane (stage)
        let new_top = aspect_ratio_y * 0.5 * new_scale;

        for (projection_3d, transform_3d) in perspective_query.iter_mut() {
            let camera_z_pos = &transform_3d.translation.z;
            let new_fov = 2.0 * (new_top / camera_z_pos).atan();

            if let Perspective(proj) = projection_3d.into_inner() {
                proj.fov = new_fov;
                // proj.fov is actually clamped by 2d camera scale, because height used to calculate fov
                // proj.fov = proj.fov.clamp(10.0f32.to_radians(), 130.0f32.to_radians());
            }
        }
    }
}
