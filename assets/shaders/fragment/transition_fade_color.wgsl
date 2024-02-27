#import bevy_ui::ui_vertex_output::UiVertexOutput

struct MaterialTime { time: f32, }
struct MaterialColor { color: vec4<f32>, }

@group(1) @binding(0) var<uniform> time_u: MaterialTime;
@group(1) @binding(1) var<uniform> color_u: MaterialColor;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    return vec4(color_u.color.x, color_u.color.y, color_u.color.z, time_u.time);
}
