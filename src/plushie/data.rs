use bevy::prelude::*;

use crate::plushie::{DisplayMode, shaders::LinkMaterial};

/// Represents a node of the graph affected by the forces acting on the weaved yarn.
/// Commonly, this represents the "V" created after completing a single crochet and similar stitches.
/// Virtual nodes are also GraphNodes, e.g. the node representing the start of a Magic Ring.
/// Note: Centroids are not GraphNodes, they are just a tool for inflating the graph.
#[derive(Component)]
pub struct GraphNode {
    pub child_selection_indicator: Entity,
    pub child_per_display_mode: enum_map::EnumMap<DisplayMode, Entity>,
}

/// Link between two GraphNodes, where the yarn exerts LinkForce
#[derive(Component)]
pub struct Link {
    pub node_a: Entity,
    pub node_b: Entity,
    pub tension: f32,
    // TODO desired length multiplier for virtual nodes at magic ring
    pub child_per_display_mode: enum_map::EnumMap<DisplayMode, Entity>,
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
    pub node_mesh: Handle<Mesh>,
    pub node_material: Handle<StandardMaterial>,
    pub selected_node_material: Handle<StandardMaterial>,
    pub link_mesh: Handle<Mesh>,
    pub link_material: Handle<StandardMaterial>,
    pub force_responding_material: Handle<LinkMaterial>,
}

#[derive(Message)]
pub struct AddGraphNode {
    pub position: Vec3,
}

#[derive(Message)]
pub struct BuildPlushieFromPattern {
    pub pattern: String,
}
