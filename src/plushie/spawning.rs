use bevy::prelude::*;
use crochet::force_graph::Initializer;
use crochet::{ColorRgb, Peculiarity, PlushieDef};
use enum_map::enum_map;

use crate::HOOK_SIZE;
use crate::plushie::DisplayMode;
use crate::plushie::animation::{
    Centroid, LinkForce, NewPosition, OriginNode, SingleLoopForce, StuffingForce,
};
use crate::plushie::data::Link;
use crate::plushie::display_mode::{DisplayPresets, select_displayed_child};
use crate::plushie::{
    data::{AddGraphNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click,
};
use crate::state::editor_simulation_sync::EditorSimulationSync;
use crate::ui::code_editor::highlighter::HighlightLayer;
use crate::ui::code_editor::messages::BuildPlushieFromPattern;
use crate::ui::code_editor::state::CodeEditorState;
use crate::ui::{ConsoleMessage, ConsolePipe, SimulationState};

fn force_display_node_color(peculiarity: Option<Peculiarity>) -> ColorRgb {
    if let Some(peculiarity) = peculiarity {
        match peculiarity {
            Peculiarity::Locked => [0, 255, 255],
            Peculiarity::Tip => [0, 127, 255],
            Peculiarity::BLO(_) => [255, 0, 0],
            Peculiarity::FLO(_) => [0, 255, 0],
        }
    } else {
        [255, 255, 255]
    }
}

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
            MeshMaterial3d(assets.get_or_create_fabric_material(
                force_display_node_color(msg.peculiarity),
                materials,
            )),
            Transform::default().with_scale(Vec3::splat(0.3)),
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
                peculiarity: msg.peculiarity,
                origin: msg.origin,
            },
            Name::new("GraphNode"),
            Transform::from_translation(msg.position).with_scale(Vec3::splat(HOOK_SIZE)),
            Pickable::default(),
            LinkForce(Vec3::ZERO),
            StuffingForce(Vec3::ZERO),
            SingleLoopForce(Vec3::ZERO),
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

fn add_centroid(commands: &mut Commands, assets: &PlushieAssets) {
    commands.spawn((
        Centroid,
        Name::new("Centroid"),
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.centroid_material.clone()),
        Transform::from_scale(Vec3::splat(HOOK_SIZE)), // does not necessarily have to be equal to hook size
        Pickable::default(),
    ));
}

pub fn adjust_centroid_number(
    mut commands: Commands,
    state: Res<SimulationState>,
    existing_centroids: Query<Entity, With<Centroid>>,
    assets: Res<PlushieAssets>,
) {
    let new_count = state.centroids as usize;
    let existing = existing_centroids.iter().len();
    if new_count > existing {
        for _ in 0..(new_count - existing) {
            add_centroid(&mut commands, &assets);
        }
    } else {
        for entity in existing_centroids.iter().skip(new_count) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn build_plushie_from_pattern(
    mut msgr: MessageReader<BuildPlushieFromPattern>,
    mut commands: Commands,
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sync_state: ResMut<EditorSimulationSync>,
    mut code_editor: ResMut<CodeEditorState>,
    state: Res<SimulationState>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>, With<Centroid>)>>,
) -> Result {
    let Some(msg) = msgr.read().last() else {
        return Ok(());
    };

    for entity in existing_plushie_entities {
        commands.entity(entity).despawn();
    }

    let pattern: crochet::Pattern = match crochet::acl_to_pattern(&msg.acl) {
        Ok(new_pattern) => new_pattern,
        Err(error) => {
            let _ = pipe.sender.send(ConsoleMessage {
                text: format!("Error in the pattern: {}", error),
            });
            // TODO errors reported here are useless, need to refactor grammar and parser
            // TODO display the error on hover (see poc in code_editor/mod.rs egui::Id::new("token_tooltip"))
            code_editor.highlighter.set(
                HighlightLayer::RedUnderline,
                vec![(error.byte_range.0..error.byte_range.1)],
            );
            return Ok(());
        }
    };

    let plushie_def: PlushieDef =
        // TODO use iterator instead of flow, no need to clone
        match crochet::Hook::parse(pattern.clone(), crochet::HookParams::default()) {
            Ok(graph) => PlushieDef {
                edges: graph.edges.into(),
                nodes: graph.nodes,
            },
            Err(error) => {
                let _ = pipe.sender.send(ConsoleMessage {
                    text: format!("Error in the pattern (hook): {:?}", error),
                });
                return Ok(());
            }
        };
    sync_state.acl_in_simulation = Some(msg.acl.clone());
    // sync_state.pattern_in_simulation = Some(pattern);
    sync_state.in_sync = true;

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
                    peculiarity: node.peculiarity,
                    origin: node.origin,
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

    for _ in 0..state.centroids {
        add_centroid(&mut commands, &assets);
    }

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Built a plushie".into(),
    });

    Ok(())
}
