struct CustomMaterial {
    start: vec4<f32>,
    stop: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    return mix(material.start, material.stop, uv.y);
}
