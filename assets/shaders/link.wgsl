#import bevy_pbr::mesh_functions::get_world_from_local
#import bevy_pbr::view_transformations::position_world_to_clip

struct LinkInstance {
    transform: mat4x4<f32>,
    force: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> instances: array<LinkInstance>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var<uniform> max_force: f32;

struct VertexInput {
    @builtin(instance_index) instance_idx: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

fn force_to_color(t: f32) -> vec4<f32> {
    let clamped = clamp(t, 0.0, 1.0);
    let cold = vec4<f32>(0.2, 0.4, 1.0, 1.0);
    let hot  = vec4<f32>(1.0, 0.15, 0.05, 1.0);
    return mix(cold, hot, clamped);
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let instance = instances[in.instance_idx];
    let world_position = instance.transform * vec4<f32>(in.position, 1.0);

    var out: VertexOutput;
    out.clip_position = position_world_to_clip(world_position.xyz);
    out.color = force_to_color(instance.force / max_force);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}