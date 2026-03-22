use bevy::prelude::*;

use crate::plushie::BuildPlushieFromPattern;
use crate::plushie::animation::LinkForce;
use crate::plushie::data::Link;
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
            Mesh3d(assets.stitch_mesh.clone()),
            MeshMaterial3d(assets.stitch_material.clone()),
            Transform::from_translation(msg.position).with_scale(Vec3::splat(radius)),
            Pickable::default(),
            LinkForce(Vec3::ZERO),
        ))
        .observe(on_click)
        .id()
}

pub fn add_link_between(a: Entity, b: Entity, commands: &mut Commands, assets: &PlushieAssets) {
    commands.spawn((
        Link { a, b },
        Mesh3d(assets.link_mesh.clone()),
        MeshMaterial3d(assets.link_material.clone()),
        Transform::default(),
    ));
}

pub fn build_plushie_from_pattern(
    mut msgr: MessageReader<BuildPlushieFromPattern>,
    mut commands: Commands,
    plushie_assets: Res<PlushieAssets>,
    pipe: Res<ConsolePipe>,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>)>>,
) -> Result {
    let Some(msg) = msgr.read().last() else {
        return Ok(());
    };

    let Some((graph_nodes, edges)) = crochet::parse(&msg.pattern) else {
        let _ = pipe.sender.send(ConsoleMessage {
            text: "Error in the pattern".into(),
        });
        return Ok(());
    };

    for entity in existing_plushie_entities {
        commands.entity(entity).despawn();
    }

    let node_entities: Vec<Entity> = graph_nodes
        .into_iter()
        .map(|node| add_graph_node(&AddNode { position: node }, &mut commands, &plushie_assets))
        .collect();

    for (source, targets) in edges.iter().enumerate() {
        for target in targets {
            let a = node_entities[source];
            let b = node_entities[*target];
            add_link_between(a, b, &mut commands, &plushie_assets);
        }
    }

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Built a plushie".into(),
    });

    Ok(())
}
