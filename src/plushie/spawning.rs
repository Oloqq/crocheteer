use bevy::prelude::*;

use crate::plushie::animation::LinkForce;
use crate::plushie::{
    data::{AddNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click,
};

pub fn add_graph_node(msg: &AddNode, commands: &mut Commands, assets: &PlushieAssets) -> Entity {
    let radius = 0.001;
    commands
        .spawn((
            GraphNode {},
            Name::new("Node"),
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.material.clone()),
            Transform::from_translation(msg.position).with_scale(Vec3::splat(radius)),
            Pickable::default(),
            LinkForce(Vec3::ZERO),
        ))
        .observe(on_click)
        .id()
}

pub fn add_new_nodes(
    mut commands: Commands,
    mut msgr: MessageReader<AddNode>,
    assets: Res<PlushieAssets>,
) {
    for msg in msgr.read() {
        add_graph_node(&msg, &mut commands, &assets);
    }
}
