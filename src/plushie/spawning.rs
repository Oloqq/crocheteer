use bevy::prelude::*;
use crochet::{ColorRgb, PlushieDef};
use enum_map::enum_map;

use crate::HOOK_SIZE;
use crate::plushie::DisplayMode;
use crate::plushie::animation::{
    Centroid, LinkForce, NewPosition, OriginNode, SingleLoopForce, StuffingForce,
};
use crate::plushie::data::{Link, OneByOneProgress};
use crate::plushie::display_mode::{DisplayPresets, select_displayed_child};
use crate::plushie::{
    data::{AddGraphNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click,
};
use crate::state::editor_simulation_sync::EditorSimulationSync;
use crate::state::simulated_plushie::PlushieInSimulation;
use crate::ui::code_editor::highlighter::{HighlightLayer, Highlighter};
use crate::ui::code_editor::messages::BuildPlushieFromPattern;
use crate::ui::code_editor::state::CodeEditorState;
use crate::ui::{ConsoleMessage, ConsolePipe, SimulationState};

fn force_display_node_color(peculiarity: Option<crochet::data::Peculiarity>) -> ColorRgb {
    if let Some(peculiarity) = peculiarity {
        match peculiarity {
            crochet::data::Peculiarity::Locked => [0, 255, 255],
            crochet::data::Peculiarity::Tip => [0, 127, 255],
            crochet::data::Peculiarity::BLO(_) => [255, 0, 0],
            crochet::data::Peculiarity::FLO(_) => [0, 255, 0],
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

pub fn ordered_plushie_build(mut msgr: MessageReader<BuildPlushieFromPattern>) -> bool {
    msgr.read().last().is_some()
}

fn despawn_old_plushie(
    commands: &mut Commands,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>, With<Centroid>)>>,
) {
    for entity in existing_plushie_entities {
        commands.entity(entity).despawn();
    }
}

fn parse_to_plushie_def(
    acl_source: &str,
    code_highlighter: &mut Highlighter,
    console_pipe: &ConsolePipe,
) -> Option<PlushieDef> {
    let plushie_def = match crochet::parse(acl_source) {
        Ok(p) => p,
        Err(error) => {
            let _ = console_pipe.sender.send(ConsoleMessage {
                text: format!("Error in pattern: {}", error),
            });
            // TODO display the error on hover (see poc in code_editor/mod.rs egui::Id::new("token_tooltip"))
            // TODO stop displaying error when text changes
            if let Some(origin) = error.origin() {
                code_highlighter.set(HighlightLayer::RedUnderline, vec![(origin.as_range())]);
            }
            return None;
        }
    };

    if plushie_def.nodes.len() == 0 {
        let _ = console_pipe.sender.send(ConsoleMessage {
            text: format!("Produced 0 nodes"),
        });
        return None;
    }

    Some(plushie_def)
}

pub fn build_full_plushie_from_pattern(
    mut msgr: MessageReader<BuildPlushieFromPattern>,
    mut commands: Commands,
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sync_state: ResMut<EditorSimulationSync>,
    mut code_editor: ResMut<CodeEditorState>,
    mut state: ResMut<SimulationState>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>, With<Centroid>)>>,
) -> Result {
    let Some(msg) = msgr.read().last() else {
        return Ok(());
    };
    match state.initializer {
        crochet::force_graph::Initializer::RegularCylinder(_) => (),
        crochet::force_graph::Initializer::OneByOne => return Ok(()),
    }
    despawn_old_plushie(&mut commands, existing_plushie_entities);
    let Some(plushie_def) = parse_to_plushie_def(&msg.acl, &mut code_editor.highlighter, &pipe)
    else {
        return Ok(());
    };

    let node_positions = state
        .initializer
        .apply(plushie_def.nodes.len() as u32, HOOK_SIZE);
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

    sync_state.plushie_parsed(msg.acl.clone());
    state.active_part = Some(plushie_def.pattern.parts[0].name.clone());
    commands.insert_resource(PlushieInSimulation {
        plushie: plushie_def,
    });
    let _ = pipe.sender.send(ConsoleMessage {
        text: "Built a plushie".into(),
    });

    Ok(())
}

pub fn start_building_plushie_one_by_one(
    mut msgr: MessageReader<BuildPlushieFromPattern>,
    mut commands: Commands,
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sync_state: ResMut<EditorSimulationSync>,
    mut code_editor: ResMut<CodeEditorState>,
    mut state: ResMut<SimulationState>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
    existing_plushie_entities: Query<Entity, Or<(With<GraphNode>, With<Link>, With<Centroid>)>>,
) -> Result {
    let Some(msg) = msgr.read().last() else {
        return Ok(());
    };
    match state.initializer {
        crochet::force_graph::Initializer::OneByOne => (),
        crochet::force_graph::Initializer::RegularCylinder(_) => return Ok(()),
    }

    despawn_old_plushie(&mut commands, existing_plushie_entities);
    let Some(plushie_def) = parse_to_plushie_def(&msg.acl, &mut code_editor.highlighter, &pipe)
    else {
        return Ok(());
    };

    assert!(plushie_def.nodes.len() == plushie_def.edges.len());
    assert!(plushie_def.nodes.len() > 0);
    let starting_nodes_positions = state
        .initializer
        .apply(plushie_def.nodes.len() as u32, HOOK_SIZE);
    let node_count = starting_nodes_positions.len();

    let node_entities: Vec<Entity> = starting_nodes_positions
        .iter()
        .zip(plushie_def.nodes.iter())
        .map(|(position, node)| {
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

    if let Some(first) = node_entities.first() {
        commands.entity(*first).insert(OriginNode);
    }

    for (source, targets) in plushie_def.edges.iter().take(node_count).enumerate() {
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

    commands.insert_resource(OneByOneProgress {
        full_plushie: plushie_def.clone(),
        next: node_count,
        node_entities,
    });
    state.active_part = Some(plushie_def.pattern.parts[0].name.clone());
    commands.insert_resource(PlushieInSimulation {
        plushie: plushie_def,
    });
    sync_state.plushie_parsed(msg.acl.clone());

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Started building a plushie one by one".into(),
    });

    Ok(())
}

pub fn continue_building_one_by_one(
    mut progress: ResMut<OneByOneProgress>,
    mut commands: Commands,
    // mut node_adder: MessageWriter<AddGraphNode>,
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
    existing: Query<&Transform>,
) {
    let plushie_def = &progress.full_plushie;
    let new_index = progress.next;
    if new_index >= plushie_def.nodes.len() {
        pipe.write("finished building a plushie one by one");
        commands.remove_resource::<OneByOneProgress>();
        return;
    }

    let new_node_definition = &plushie_def.nodes[new_index];
    let new_edges = &plushie_def.edges[new_index];
    let position_basis_entities: Vec<Entity> = new_edges
        .iter()
        .filter_map(|i| progress.node_entities.get(*i).copied())
        .collect();
    let position_basis: Vec<Vec3> = position_basis_entities
        .iter()
        .filter_map(|entity| existing.get(*entity).map(|e| e.translation).ok())
        .collect();

    let new_node_entity = add_graph_node(
        &AddGraphNode {
            position: new_node_position(&position_basis),
            color: new_node_definition.color,
            peculiarity: new_node_definition.peculiarity,
            origin: new_node_definition.origin,
        },
        &mut commands,
        &mut assets,
        &mut materials,
        &display_presets,
    );

    let source = new_index;
    for target in new_edges {
        let a = new_node_entity;
        let b = progress.node_entities[*target];
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
    progress.node_entities.push(new_node_entity);
    progress.next += 1;
}

fn new_node_position(based_on: &Vec<Vec3>) -> Vec3 {
    if based_on.len() == 0 {
        unreachable!()
    } else if based_on.len() == 1 {
        based_on[0] + Vec3::new(0.0, HOOK_SIZE, 0.0)
    } else {
        let mut avg = Vec3::ZERO;
        for base in based_on {
            avg += base;
        }
        avg /= based_on.len() as f32;
        // TODO addition of HOOK_SIZE to Y can behave weird if work transitions from building vertically to horizontally
        // this is needed for now to introduce variation third dimension, otherwise nodes settle on a plane
        // ideally, implementation would be completely agnostic to orientation
        // the "working horizontally" thing could be solved by using vector from parent to current node here
        // the issue of introducing third dimension still needs to be addressed then
        avg += Vec3::new(0.0, HOOK_SIZE, 0.0);
        avg
    }
}
