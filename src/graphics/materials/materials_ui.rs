use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// Normal UI Material ----------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct NormalButtonMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl UiMaterial for NormalButtonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/ui_normal_button.wgsl".into()
    }
}

// Focused UI Material ---------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct FocusedButtonMaterial {
    #[uniform(0)]
    pub time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl UiMaterial for FocusedButtonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fragment/ui_focused_button.wgsl".into()
    }
}
