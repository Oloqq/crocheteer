use std::collections::HashMap;

use bevy::prelude::*;
use crochet::{ColorRgb, PlushieDef};

use crate::plushie::{DisplayMode, shaders::LinkMaterial};

/// Represents a node of the graph affected by the forces acting on the weaved yarn.
/// Commonly, this represents the "V" created after completing a single crochet and similar stitches.
/// Virtual nodes are also GraphNodes, e.g. the node representing the start of a Magic Ring.
/// Note: Centroids are not GraphNodes, they are just a tool for inflating the graph.
#[derive(Component)]
pub struct GraphNode {
    pub child_selection_indicator: Entity,
    pub child_per_display_mode: enum_map::EnumMap<DisplayMode, Entity>,
    pub peculiarity: Option<crochet::data::Peculiarity>,
    pub origin: Option<crochet::acl::Origin>,
    pub part_index: usize,
}

/// Link between two GraphNodes, where the yarn exerts LinkForce
#[derive(Component)]
pub struct Link {
    pub node_a: Entity,
    pub node_b: Entity,
    pub tension: f32,
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
    pub link_mesh: Handle<Mesh>,
    pub colored_materials: HashMap<ColorRgb, Handle<StandardMaterial>>,
    pub selected_node_material: Handle<StandardMaterial>,
    pub force_responding_material: Handle<LinkMaterial>,
    pub centroid_material: Handle<StandardMaterial>,
}

impl PlushieAssets {
    pub fn get_or_create_fabric_material(
        &mut self,
        color: ColorRgb,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if self.colored_materials.contains_key(&color) {
            self.colored_materials[&color].clone()
        } else {
            let float_color = [
                color[0] as f32 / 255.0,
                color[1] as f32 / 255.0,
                color[2] as f32 / 255.0,
            ];
            let mut material = StandardMaterial::from_color(Color::srgb_from_array(float_color));
            material.perceptual_roughness = 0.7;
            let handle = materials.add(material);
            self.colored_materials.insert(color, handle.clone());
            handle
        }
    }
}

#[derive(Message)]
pub struct AddGraphNode {
    pub position: Vec3,
    pub color: ColorRgb,
    pub peculiarity: Option<crochet::data::Peculiarity>,
    pub origin: Option<crochet::acl::Origin>,
    pub node_index: usize,
    pub part_index: usize,
}

// TEMP
#[derive(Resource)]
pub struct OneByOneProgress {}
