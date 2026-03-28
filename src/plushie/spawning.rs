use bevy::prelude::*;
use crochet::Initializer;
use enum_map::enum_map;

use crate::plushie::animation::{Centroid, LinkForce, NewPosition, OriginNode, StuffingForce};
use crate::plushie::data::Link;
use crate::plushie::display_mode::{DisplayPresets, select_displayed_child};
use crate::plushie::{BuildPlushieFromPattern, DisplayMode};
use crate::plushie::{
    data::{AddGraphNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click,
};
use crate::ui::{ConsoleMessage, ConsolePipe};

fn add_graph_node(
    msg: &AddGraphNode,
    commands: &mut Commands,
    assets: &PlushieAssets,
    presets: &DisplayPresets,
) -> Entity {
    commands
        .spawn((
            GraphNode {},
            Name::new("GraphNode"),
            Mesh3d(assets.node_mesh.clone()),
            MeshMaterial3d(assets.node_material.clone()),
            Transform::from_translation(msg.position)
                .with_scale(Vec3::splat(presets.current().node_radius)),
            Pickable::default(),
            LinkForce(Vec3::ZERO),
            StuffingForce(Vec3::ZERO),
        ))
        .observe(on_click)
        .id()
}

fn add_link_between(
    node_a: Entity,
    node_b: Entity,
    commands: &mut Commands,
    assets: &PlushieAssets,
    presets: &DisplayPresets,
) {
    let standard_material_child: Entity = commands
        .spawn((
            Visibility::Hidden,
            Mesh3d(assets.link_mesh.clone()),
            MeshMaterial3d(assets.link_material.clone()),
        ))
        .id();
    let shader_material_child: Entity = commands
        .spawn((
            Visibility::Hidden,
            Mesh3d(assets.link_mesh.clone()),
            MeshMaterial3d(assets.force_responding_material.clone()),
        ))
        .id();

    let child_per_display_mode = enum_map! {
        DisplayMode::Pattern => standard_material_child,
        DisplayMode::Forces => shader_material_child
    };
    select_displayed_child(commands, &child_per_display_mode, presets.current_mode);

    commands
        .spawn((
            Link {
                node_a,
                node_b,
                tension: 0.0,
                child_per_display_mode,
            },
            Transform::default(),
        ))
        .add_children(&[standard_material_child, shader_material_child]);
}

pub fn build_plushie_from_pattern(
    mut msgr: MessageReader<BuildPlushieFromPattern>,
    mut commands: Commands,
    assets: Res<PlushieAssets>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>, With<Centroid>)>>,
) -> Result {
    let Some(msg) = msgr.read().last() else {
        return Ok(());
    };

    let Some(plushie_def) = crochet::parse(&msg.pattern) else {
        let _ = pipe.sender.send(ConsoleMessage {
            text: "Error in the pattern".into(),
        });
        return Ok(());
    };

    for entity in existing_plushie_entities {
        commands.entity(entity).despawn();
    }

    let hook_size = 5e-4;
    let node_positions =
        Initializer::RegularCylinder(12).apply(plushie_def.nodes.len() as u32, hook_size);
    let node_entities: Vec<Entity> = node_positions
        .into_iter()
        .map(|node| {
            add_graph_node(
                &AddGraphNode { position: node },
                &mut commands,
                &assets,
                &display_presets,
            )
        })
        .collect();

    // assumption: first is the virtual node of magic ring
    // TODO differentiate virtual node in display
    // this is required because centroids cause creations to drift away is there isn't any anchor point
    if let Some(first) = node_entities.first() {
        commands.entity(*first).insert(OriginNode);
    }

    for (source, targets) in plushie_def.edges.iter().enumerate() {
        for target in targets {
            let a = node_entities[source];
            let b = node_entities[*target];
            add_link_between(a, b, &mut commands, &assets, &display_presets);
        }
    }

    let radius = 0.001;
    commands.spawn((
        Centroid,
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.selected_node_material.clone()),
        Transform::from_translation(Vec3::new(0.0, 4e-3, 0.0)).with_scale(Vec3::splat(radius)),
        Pickable::default(),
    ));
    commands.spawn((
        Centroid,
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.selected_node_material.clone()),
        Transform::from_translation(Vec3::new(0.0, 4e-3, 0.0)).with_scale(Vec3::splat(radius)),
        Pickable::default(),
    ));

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Built a plushie".into(),
    });

    Ok(())
}
