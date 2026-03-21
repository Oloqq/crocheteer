use bevy::prelude::*;

use crate::plushie::{
    data::{AddNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click_ball,
};

pub fn add_new_nodes(
    mut commands: Commands,
    mut msgr: MessageReader<AddNode>,
    assets: Res<PlushieAssets>,
) {
    let radius = 0.001;
    for msg in msgr.read() {
        commands
            .spawn((
                GraphNode {},
                Name::new("Node"),
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.material.clone()),
                Transform::from_translation(msg.position).with_scale(Vec3::splat(radius)),
                Pickable::default(),
            ))
            .observe(on_click_ball);
    }
}
