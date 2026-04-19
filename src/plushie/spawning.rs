use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crochet::force_graph::simulated_plushie::init::OneByOneResult;
use crochet::{ColorRgb, PlushieDef};
use enum_map::enum_map;

use crate::HOOK_SIZE;
use crate::plushie::DisplayMode;
use crate::plushie::animation::{Centroid, LinkForce, SingleLoopForce, StuffingForce};
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
    index_to_entity: &mut HashMap<usize, Entity>,
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

    let entity = commands
        .spawn((
            GraphNode {
                child_per_display_mode,
                child_selection_indicator,
                peculiarity: msg.peculiarity,
                origin: msg.origin,
                part_index: msg.part_index,
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
        .id();

    index_to_entity.insert(msg.node_index, entity);
    entity
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
    // TODO merge this and parse_to_plushie_def
    let simulated_plushie =
        crochet::parse_to_simulated(&msg.acl, HOOK_SIZE, &state.initializer).unwrap();
    let mut index_to_entity = HashMap::new();

    let node_entities: Vec<Entity> = simulated_plushie
        .nodes()
        .iter()
        .enumerate()
        .map(|(node_index, node)| {
            add_graph_node(
                &AddGraphNode {
                    position: node.position.clone(),
                    color: node.definition.color,
                    peculiarity: node.definition.peculiarity,
                    origin: node.definition.origin,
                    part_index: node.definition.part_index,
                    node_index,
                },
                &mut commands,
                &mut assets,
                &mut materials,
                &display_presets,
                &mut index_to_entity,
            )
        })
        .collect();

    // assumption: first is the virtual node of magic ring
    // this is required because centroids cause creations to drift away is there isn't any anchor point
    // if let Some(first) = node_entities.first() {
    //     commands.entity(*first).insert(OriginNode);
    // }

    for (source, targets) in simulated_plushie.edges().iter().enumerate() {
        for target in targets {
            let a = node_entities[source];
            let b = node_entities[*target];
            add_link_between(
                a,
                b,
                &mut commands,
                &mut assets,
                &mut materials,
                simulated_plushie.nodes()[source].definition.color,
                &display_presets,
            );
        }
    }

    commands.insert_resource(PlushieInSimulation {
        definition: plushie_def.clone(),
        plushie: simulated_plushie.clone(),
        index_to_entity,
    });

    sync_state.plushie_parsed(msg.acl.clone());
    state.active_part = Some(simulated_plushie.parts()[0].name.clone());
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
    // TODO merge this and parse_to_plushie_def, remove clones
    let simulated_plushie =
        crochet::parse_to_simulated(&msg.acl, HOOK_SIZE, &state.initializer).unwrap();
    let mut index_to_entity = HashMap::new();

    let node_entities: Vec<Entity> = simulated_plushie
        .nodes()
        .iter()
        .enumerate()
        .map(|(node_index, node)| {
            add_graph_node(
                &AddGraphNode {
                    position: node.position.clone(),
                    color: node.definition.color,
                    peculiarity: node.definition.peculiarity,
                    origin: node.definition.origin,
                    part_index: node.definition.part_index,
                    node_index,
                },
                &mut commands,
                &mut assets,
                &mut materials,
                &display_presets,
                &mut index_to_entity,
            )
        })
        .collect();

    // if let Some(first) = node_entities.first() {
    //     commands.entity(*first).insert(OriginNode);
    // }

    for (source, targets) in simulated_plushie.edges().iter().enumerate() {
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

    commands.insert_resource(PlushieInSimulation {
        definition: plushie_def.clone(),
        plushie: simulated_plushie.clone(),
        index_to_entity,
    });

    commands.insert_resource(OneByOneProgress {});
    state.active_part = Some(plushie_def.pattern.parts[0].name.clone());
    sync_state.plushie_parsed(msg.acl.clone());

    let _ = pipe.sender.send(ConsoleMessage {
        text: "Started building a plushie one by one".into(),
    });

    Ok(())
}

pub fn continue_building_one_by_one(
    mut plushie: ResMut<PlushieInSimulation>,
    _: Res<OneByOneProgress>,
    mut commands: Commands,
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
) -> Result {
    match plushie.plushie.advance_one_by_one() {
        OneByOneResult::Finished => {
            pipe.write("finished building a plushie one by one");
            commands.remove_resource::<OneByOneProgress>();
            return Ok(());
        }
        OneByOneResult::Advanced(new_index) => {
            let new_node = &plushie.plushie.nodes()[new_index];
            let msg = AddGraphNode {
                position: new_node.position,
                color: new_node.definition.color,
                peculiarity: new_node.definition.peculiarity,
                origin: new_node.definition.origin,
                node_index: new_index,
                part_index: new_node.definition.part_index,
            };
            let new_node_entity = add_graph_node(
                &msg,
                &mut commands,
                &mut assets,
                &mut materials,
                &display_presets,
                &mut plushie.index_to_entity,
            );

            let new_edges = &plushie.plushie.edges().edges_from_node(new_index);
            for target in new_edges.iter() {
                let a = new_node_entity;
                let b = plushie
                    .index_to_entity
                    .get(target)
                    .expect("index to entity should contain lesser-index node");
                add_link_between(
                    a,
                    *b,
                    &mut commands,
                    &mut assets,
                    &mut materials,
                    plushie.plushie.nodes()[new_index].definition.color,
                    &display_presets,
                );
            }
        }
    }
    Ok(())
}
