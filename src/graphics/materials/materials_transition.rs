use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// Transition FADE TO COLOR ----------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct FadeToColorMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub color: Vec4,
}

impl UiMaterial for FadeToColorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/transition_fade_color.wgsl".into()
    }
}

// Transition FADE FROM COLOR --------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct FadeFromColorMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub color: Vec4,
}

impl UiMaterial for FadeFromColorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/transition_fade_color.wgsl".into()
    }
}
