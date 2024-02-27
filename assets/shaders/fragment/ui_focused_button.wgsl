#import bevy_ui::ui_vertex_output::UiVertexOutput

struct FocusedButtonMaterial {
    time: f32,
}
@group(1) @binding(0) var<uniform> material: FocusedButtonMaterial;
@group(1) @binding(1) var color_texture: texture_2d<f32>;
@group(1) @binding(2) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);

    let time_wave = (0.5 * sin(material.time)) + 0.5;

    return texture * vec4(time_wave, time_wave, time_wave, 1.0);
}