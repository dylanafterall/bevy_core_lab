use crate::states::app_state::AppState;

use bevy::{
    core::FrameCount,
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use bevy_framepace::{FramepacePlugin, Limiter};

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin)
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .insert_resource(Msaa::Off)
            .init_resource::<ResolutionSettings>()
            .init_resource::<FramerateSettings>()
            .add_event::<EditResolution>()
            .add_event::<EditVsync>()
            .add_event::<EditFullscreen>()
            .add_event::<EditMsaa>()
            .add_event::<EditFramerate>()
            .add_systems(PreStartup, setup_window)
            .add_systems(OnExit(AppState::Splash), set_normal_clear_color)
            .add_systems(
                Update,
                (
                    make_visible.run_if(in_state(AppState::Splash)),
                    emit_edit_resolution.run_if(in_state(AppState::Options)),
                    handle_edit_resolution.run_if(in_state(AppState::Options)),
                    handle_edit_vsync.run_if(in_state(AppState::Options)),
                    handle_edit_fullscreen.run_if(in_state(AppState::Options)),
                    handle_edit_msaa.run_if(in_state(AppState::Options)),
                    handle_edit_framerate.run_if(in_state(AppState::Options)),
                ),
            );
    }
}

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub enum AspectRatio {
    _4_3,  // _16_12
    _5_4,  // _15_12    most restrictive width
    _8_5,  // _16_10
    _16_9, // _16_9     most restrictive height
    _21_9, // _21_9
           // design game for _15_9 ratio and all resolutions will work
}

#[derive(Resource)]
pub struct ResolutionSettings {
    pub _1024_768: Vec2,  // 4:3          (XGA - Apple iPad)
    pub _1280_1024: Vec2, // 5:4          (SXGA)
    pub _1280_720: Vec2,  // 16:9         (WXGA 720p)
    pub _1280_800: Vec2,  // 8:5          (WXGA)
    pub _1360_768: Vec2,  // ~16:9
    pub _1366_768: Vec2,  // ~16:9        (WXGA HD)
    pub _1440_900: Vec2,  // 8:5          (WSXGA)
    pub _1600_900: Vec2,  // 16:9         (HD+ 900p)
    pub _1680_1050: Vec2, // 8:5          (WSXGA+)
    pub _1920_1200: Vec2, // 8:5          (WUXGA)
    pub _1920_1080: Vec2, // 16:9         (FHD 1080p)
    pub _2560_1440: Vec2, // 16:9         (WQHD 2K)
    pub _2560_1600: Vec2, // 8:5          (WQXGA)
    pub _2560_1080: Vec2, // 21:9         (UW-FHD)
    pub _2880_1800: Vec2, // 8:5          (MBP Retina)
    pub _3440_1440: Vec2, // 21:9         (UW-QHD)
    pub _3840_2160: Vec2, // 16:9         (4K UHD-1)
}

impl Default for ResolutionSettings {
    fn default() -> ResolutionSettings {
        ResolutionSettings {
            _1024_768: Vec2::new(1024.0, 768.0),
            _1280_1024: Vec2::new(1280.0, 1024.0),
            _1280_720: Vec2::new(1280.0, 720.0),
            _1280_800: Vec2::new(1280.0, 800.0),
            _1360_768: Vec2::new(1360.0, 768.0),
            _1366_768: Vec2::new(1366.0, 768.0),
            _1440_900: Vec2::new(1440.0, 900.0),
            _1600_900: Vec2::new(1600.0, 900.0),
            _1680_1050: Vec2::new(1680.0, 1050.0),
            _1920_1200: Vec2::new(1920.0, 1200.0),
            _1920_1080: Vec2::new(1920.0, 1080.0),
            _2560_1440: Vec2::new(2560.0, 1440.0),
            _2560_1600: Vec2::new(2560.0, 1600.0),
            _2560_1080: Vec2::new(2560.0, 1080.0),
            _2880_1800: Vec2::new(2880.0, 1800.0),
            _3440_1440: Vec2::new(3440.0, 1440.0),
            _3840_2160: Vec2::new(3840.0, 2160.0),
        }
    }
}

#[derive(Resource)]
pub struct FramerateSettings {
    manual_fps: f64,
}

impl Default for FramerateSettings {
    fn default() -> FramerateSettings {
        FramerateSettings { manual_fps: 60.0 }
    }
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditResolution {
    pub resolution: Vec2,
    pub aspect_ratio: AspectRatio,
}

#[derive(Event)]
pub struct EditVsync;

#[derive(Event)]
pub struct EditFullscreen;

#[derive(Event)]
pub struct EditMsaa;

#[derive(Event)]
pub struct EditFramerate;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn setup_window(
    mut windows: Query<&mut Window>,
    resolution_settings: Res<ResolutionSettings>,
    mut framerate: ResMut<bevy_framepace::FramepaceSettings>,
) {
    // TODO: replace hard coded default values with saved options in external file
    let mut window = windows.single_mut();

    window.title = "Make Like".into();
    window.present_mode = PresentMode::AutoVsync;
    window.resizable = false;

    // window.cursor.visible = false;

    let default_res = resolution_settings._2560_1440;
    window.resolution.set(default_res.x, default_res.y);
    WindowResolution::set_scale_factor_override(&mut window.resolution, Some(1.0));

    framerate.limiter = Limiter::Auto;
}

pub fn set_normal_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::PURPLE;
}

pub fn make_visible(mut windows: Query<&mut Window>, frames: Res<FrameCount>) {
    // the delay may be different for your app or system.
    if frames.0 == 3 {
        // at this point the gpu is ready to show the app so we can make the window visible
        windows.single_mut().visible = true;
    }
}

pub fn handle_edit_resolution(
    mut windows: Query<&mut Window>,
    mut read_edit_resolution: EventReader<EditResolution>,
) {
    for resolution_edit in read_edit_resolution.read() {
        let mut window = windows.single_mut();
        let new_resolution = resolution_edit.resolution;

        window.resolution.set(new_resolution.x, new_resolution.y);
        info!(
            "Resolution changed to: {:?} x {:?}",
            new_resolution.x, new_resolution.y
        );
    }
}

pub fn handle_edit_vsync(
    mut windows: Query<&mut Window>,
    mut read_edit_vsync: EventReader<EditVsync>,
) {
    for _ in read_edit_vsync.read() {
        let mut window = windows.single_mut();

        if matches!(window.present_mode, PresentMode::AutoVsync) {
            window.present_mode = PresentMode::AutoNoVsync;
            info!("Window VSync changed to: {:?}", window.present_mode);
        } else {
            window.present_mode = PresentMode::AutoVsync;
            info!("Window VSync changed to: {:?}", window.present_mode);
        };
    }
}

pub fn handle_edit_fullscreen(
    mut windows: Query<&mut Window>,
    mut read_edit_fullscreen: EventReader<EditFullscreen>,
) {
    for _ in read_edit_fullscreen.read() {
        let mut window = windows.single_mut();

        window.mode = if matches!(window.mode, WindowMode::BorderlessFullscreen) {
            WindowMode::Windowed
        } else {
            WindowMode::BorderlessFullscreen
        };
        info!("Window Mode changed to: {:?}", window.mode);
    }
}

pub fn handle_edit_msaa(
    mut commands: Commands,
    current_msaa_setting: Res<Msaa>,
    mut read_edit_msaa: EventReader<EditMsaa>,
) {
    for _ in read_edit_msaa.read() {
        match current_msaa_setting.samples() {
            1 => {
                // Off = 1
                commands.insert_resource(Msaa::Sample2);
                info!("MSAA setting changed to: {:?}", Msaa::Sample2);
            }
            2 => {
                // Sample2 = 2
                commands.insert_resource(Msaa::Sample4);
                info!("MSAA setting changed to: {:?}", Msaa::Sample4);
            }
            4 => {
                // Sample4 = 4
                commands.insert_resource(Msaa::Sample8);
                info!("MSAA setting changed to: {:?}", Msaa::Sample8);
            }
            8 => {
                // Sample8 = 8
                commands.insert_resource(Msaa::Off);
                info!("MSAA setting changed to: {:?}", Msaa::Off);
            }
            _ => {
                warn!("Invalid MSAA sample setting detected.")
            }
        }
    }
}

pub fn handle_edit_framerate(
    windows: Query<&Window>,
    mut framerate: ResMut<bevy_framepace::FramepaceSettings>,
    limiter_setting: Res<FramerateSettings>,
    mut read_edit_framerate: EventReader<EditFramerate>,
) {
    for _ in read_edit_framerate.read() {
        let window = windows.single();
        if matches!(window.present_mode, PresentMode::AutoVsync) {
            info!("Cannot edit Framerate limiter while VSync is enabled.");
            return;
        }

        match framerate.limiter {
            Limiter::Auto => {
                framerate.limiter = Limiter::from_framerate(limiter_setting.manual_fps);
                info!(
                    "Framerate limiter set to Manual, {:?} fps.",
                    limiter_setting.manual_fps
                );
            }
            Limiter::Manual(_duration) => {
                framerate.limiter = Limiter::Off;
                info!("Framerate limiter set to Off.");
            }
            Limiter::Off => {
                framerate.limiter = Limiter::Auto;
                info!("Framerate limiter set to Auto.");
            }
        }
    }
}

// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// TODO: replace the functions below with proper ui elements for users
// -----------------------------------------------------------------------------
pub fn emit_edit_resolution(
    windows: Query<&mut Window>,
    resolution_settings: Res<ResolutionSettings>,
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    mut write_edit_resolution: EventWriter<EditResolution>,
) {
    if key_input.just_pressed(KeyCode::Digit0) {
        let window = &windows.single().resolution;

        match (window.physical_width(), window.physical_height()) {
            (1024, 768) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1280_1024,
                    aspect_ratio: AspectRatio::_5_4,
                });
            }
            (1280, 1024) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1280_720,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (1280, 720) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1280_800,
                    aspect_ratio: AspectRatio::_8_5,
                });
            }
            (1280, 800) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1360_768,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (1360, 768) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1366_768,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (1366, 768) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1440_900,
                    aspect_ratio: AspectRatio::_8_5,
                });
            }
            (1440, 900) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1600_900,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (1600, 900) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1680_1050,
                    aspect_ratio: AspectRatio::_8_5,
                });
            }
            (1680, 1050) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1920_1200,
                    aspect_ratio: AspectRatio::_8_5,
                });
            }
            (1920, 1200) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1920_1080,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (1920, 1080) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._2560_1440,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (2560, 1440) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._2560_1600,
                    aspect_ratio: AspectRatio::_8_5,
                });
            }
            (2560, 1600) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._2560_1080,
                    aspect_ratio: AspectRatio::_21_9,
                });
            }
            (2560, 1080) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._2880_1800,
                    aspect_ratio: AspectRatio::_8_5,
                });
            }
            (2880, 1800) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._3440_1440,
                    aspect_ratio: AspectRatio::_21_9,
                });
            }
            (3440, 1440) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._3840_2160,
                    aspect_ratio: AspectRatio::_16_9,
                });
            }
            (3840, 2160) => {
                write_edit_resolution.send(EditResolution {
                    resolution: resolution_settings._1024_768,
                    aspect_ratio: AspectRatio::_4_3,
                });
            }
            _ => {
                warn!("Invalid resolution detected.")
            }
        }
        key_input.reset(KeyCode::Digit0);
    }
}
