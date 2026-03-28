use bevy::prelude::*;
use crochet::{ColorRgb, Initializer, PlushieDef};
use enum_map::enum_map;

use crate::HOOK_SIZE;
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
    assets: &mut PlushieAssets,
    materials: &mut Assets<StandardMaterial>,
    presets: &DisplayPresets,
) -> Entity {
    let pattern_child: Entity = commands
        .spawn((
            Visibility::Hidden,
            Mesh3d(assets.node_mesh.clone()),
            MeshMaterial3d(assets.get_or_create_fabric_material(msg.color, materials)),
        ))
        .id();
    let force_child: Entity = commands
        .spawn((
            Visibility::Hidden,
            Mesh3d(assets.node_mesh.clone()),
            MeshMaterial3d(assets.get_or_create_fabric_material(msg.color, materials)),
            Transform::default().with_scale(Vec3::splat(0.5)),
        ))
        .id();
    let child_selection_indicator: Entity = commands
        .spawn((
            Visibility::Hidden,
            Mesh3d(assets.node_mesh.clone()),
            MeshMaterial3d(assets.selected_node_material.clone()),
            Transform::default().with_scale(Vec3::splat(1.1)),
        ))
        .id();

    let child_per_display_mode = enum_map! {
        DisplayMode::Pattern => pattern_child,
        DisplayMode::Forces => force_child
    };
    select_displayed_child(commands, &child_per_display_mode, presets.current_mode);

    commands
        .spawn((
            GraphNode {
                child_per_display_mode,
                child_selection_indicator,
            },
            Name::new("GraphNode"),
            Transform::from_translation(msg.position).with_scale(Vec3::splat(HOOK_SIZE)),
            Pickable::default(),
            LinkForce(Vec3::ZERO),
            StuffingForce(Vec3::ZERO),
        ))
        .add_children(&[child_selection_indicator, pattern_child, force_child])
        .observe(on_click)
        .id()
}

fn add_link_between(
    node_a: Entity,
    node_b: Entity,
    commands: &mut Commands,
    assets: &mut PlushieAssets,
    materials: &mut Assets<StandardMaterial>,
    color: ColorRgb,
    presets: &DisplayPresets,
) {
    let standard_material_child: Entity = commands
        .spawn((
            Visibility::Hidden,
            Mesh3d(assets.link_mesh.clone()),
            MeshMaterial3d(assets.get_or_create_fabric_material(color, materials)),
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
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>, With<Centroid>)>>,
) -> Result {
    let Some(msg) = msgr.read().last() else {
        return Ok(());
    };

    let plushie_def: PlushieDef = match crochet::parse(&msg.pattern) {
        Some(p) => p,
        None => {
            let _ = pipe.sender.send(ConsoleMessage {
                text: "Error in the pattern".into(),
            });
            return Ok(());
        }
    };

    for entity in existing_plushie_entities {
        commands.entity(entity).despawn();
    }

    let node_positions =
        Initializer::RegularCylinder(12).apply(plushie_def.nodes.len() as u32, HOOK_SIZE);
    assert!(plushie_def.nodes.len() == node_positions.len());
    assert!(plushie_def.nodes.len() == plushie_def.edges.len());

    let node_entities: Vec<Entity> = plushie_def
        .nodes
        .iter()
        .zip(node_positions)
        .map(|(node, position)| {
            add_graph_node(
                &AddGraphNode {
                    position: position.clone(),
                    color: node.color,
                },
                &mut commands,
                &mut assets,
                &mut materials,
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
            add_link_between(
                a,
                b,
                &mut commands,
                &mut assets,
                &mut materials,
                plushie_def.nodes[source].color,
                &display_presets,
            );
        }
    }

    let radius = HOOK_SIZE; // only visual, centroid can be displayed however
    commands.spawn((
        Centroid,
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.centroid_material.clone()),
        Transform::from_translation(Vec3::new(0.0, 4e-3, 0.0)).with_scale(Vec3::splat(radius)),
        Pickable::default(),
    ));
    commands.spawn((
        Centroid,
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.centroid_material.clone()),
        Transform::from_translation(Vec3::new(0.0, 4e-3, 0.0)).with_scale(Vec3::splat(radius)),
        Pickable::default(),
    ));

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Built a plushie".into(),
    });

    Ok(())
}
