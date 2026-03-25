use bevy::prelude::*;

use crate::plushie::shaders::LinkMaterial;

/// Represents a Pull-Through, or "the little V"
#[derive(Component)]
pub struct GraphNode {}

#[derive(Component)]
pub struct Link {
    pub a: Entity,
    pub b: Entity,
    pub tension: f32,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Dragging {
    pub offset: Vec3,
    pub plane: InfinitePlane3d,
    pub plane_origin: Vec3,
}

#[derive(Resource, Default)]
pub struct PressHandled(pub bool);

#[derive(Resource)]
pub struct PlushieAssets {
    pub stitch_mesh: Handle<Mesh>,
    pub stitch_material: Handle<StandardMaterial>,
    pub selected_material: Handle<StandardMaterial>,
    pub link_mesh: Handle<Mesh>,
    pub link_material: Handle<StandardMaterial>,
    pub force_responding_material: Handle<LinkMaterial>,
}

#[derive(Message)]
pub struct AddNode {
    pub position: Vec3,
}

#[derive(Message)]
pub struct BuildPlushieFromPattern {
    pub pattern: String,
}
