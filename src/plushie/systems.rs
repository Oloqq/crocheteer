use std::collections::HashMap;

use crate::plushie::shaders::LinkMaterial;

use super::data::GraphNode;
use super::data::*;
use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut link_shader_materials: ResMut<Assets<LinkMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    // TODO https://github.com/komadori/bevy_mod_outline
    let mut selected_node_material = StandardMaterial::from_color(Color::srgb(1.0, 1.0, 1.0));
    selected_node_material.metallic = 0.7;

    let mut centroid_material = StandardMaterial::from_color(Color::srgb(1.0, 1.0, 1.0));
    centroid_material.attenuation_color = Color::linear_rgb(1.0, 0.0, 0.0);
    centroid_material.thickness = 1.0;
    centroid_material.diffuse_transmission = 0.5;

    let assets = PlushieAssets {
        node_mesh: meshes.add(Sphere::new(1.0)),
        link_mesh: meshes.add(Cylinder::new(1.0, 1.0)),
        colored_materials: HashMap::new(),
        selected_node_material: materials.add(selected_node_material),
        force_responding_material: link_shader_materials.add(LinkMaterial {
            instances: buffers.add(ShaderStorageBuffer::default()),
        }),
        centroid_material: materials.add(centroid_material),
    };
    commands.insert_resource(assets);
}

pub fn highlight_selected_nodes_visually(
    mut commands: Commands,
    mut selection_removed: RemovedComponents<Selected>,
    graph_nodes: Query<&GraphNode>,
    selection_added: Query<Entity, (With<GraphNode>, With<Selected>, Added<Selected>)>,
) {
    for entity in &selection_added {
        let graph_node = graph_nodes
            .get(entity)
            .expect("selection should be applied to just GraphNodes");
        commands
            .entity(graph_node.child_selection_indicator)
            .insert(Visibility::Visible);
    }
    for entity in selection_removed.read() {
        let graph_node = graph_nodes
            .get(entity)
            .expect("selection should be applied to just GraphNodes");
        commands
            .entity(graph_node.child_selection_indicator)
            .insert(Visibility::Hidden);
    }
}
