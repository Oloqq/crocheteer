use bevy::prelude::*;

use crate::plushie::BuildPlushieFromPattern;
use crate::plushie::animation::{LinkAssets, LinkForce, add_link_between};
use crate::plushie::{
    data::{AddNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click,
};
use crate::ui::{ConsoleMessage, ConsolePipe};

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

pub fn build_plushie_from_pattern(
    mut msgr: MessageReader<BuildPlushieFromPattern>,
    mut commands: Commands,
    plushie_assets: Res<PlushieAssets>,
    link_assets: Res<LinkAssets>,
    pipe: Res<ConsolePipe>,
) {
    let Some(msg) = msgr.read().last() else {
        return;
    };

    let (graph_nodes, edges) = crochet::parse(&msg.pattern);

    let node_entities: Vec<Entity> = graph_nodes
        .into_iter()
        .map(|node| add_graph_node(&AddNode { position: node }, &mut commands, &plushie_assets))
        .collect();

    for (source, targets) in edges.iter().enumerate() {
        for target in targets {
            let a = node_entities[source];
            let b = node_entities[*target];
            add_link_between(a, b, &mut commands, &link_assets);
        }
    }

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Built a plushie".into(),
    });
}
