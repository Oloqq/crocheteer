// TODO understand shaders

#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip
}

struct LinkInstance {
    force: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<storage, read> instances: array<LinkInstance>;

struct VertexInput {
    // instance_index must be declared as a builtin — not a location
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

fn force_to_color(t: f32) -> vec4<f32> {
    let clamped = clamp(t, -1.0, 1.0);
    let negative1 = vec4<f32>(0.58, 0.58, 1.0, 1.0);
    let zero      = vec4<f32>(0.58, 1.0, 0.58, 1.0);
    let positive1 = vec4<f32>(1.0, 0.34, 0.34, 1.0);

    let low  = mix(negative1, zero,      clamp(clamped + 1.0, 0.0, 1.0));
    let high = mix(zero,      positive1, clamp(clamped,       0.0, 1.0));
    // step returns 1.0 if first argument is smaller than second
    // so this is: if (clamped > 0) return high, else return low
    return mix(low, high, step(0.0, clamped));
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let world_from_local = mesh_functions::get_world_from_local(in.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(in.position, 1.0)
    );

    // Use get_tag() to retrieve the MeshTag, which is the buffer index
    let tag = mesh_functions::get_tag(in.instance_index);

    var out: VertexOutput;
    out.clip_position = position_world_to_clip(world_position.xyz);
    out.color = force_to_color(instances[tag].force);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}