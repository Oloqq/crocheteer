use bevy::prelude::*;
use crochet::ColorRgb;
use crochet::force_graph::simulated_plushie::init::OneByOneResult;
use enum_map::enum_map;

use crate::HOOK_SIZE;
use crate::plushie::DisplayMode;
use crate::plushie::animation::Centroid;
use crate::plushie::data::Link;
use crate::plushie::display_mode::{DisplayPresets, select_displayed_child};
use crate::plushie::{
    data::{AddGraphNode, GraphNode, PlushieAssets},
    mouse_interactions::on_click_graph_node,
};
use crate::state::editor_simulation_sync::EditorSimulationSync;
use crate::state::simulated_plushie::{NodeLookup, PlushieInSimulation};
use crate::ui::code_editor::highlighter::{HighlightLayer, Highlighter};
use crate::ui::code_editor::messages::BuildPlushieFromPattern;
use crate::ui::code_editor::state::CodeEditorState;
use crate::ui::{ConsolePipe, SimulationState};

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
    node_lookup: &mut NodeLookup,
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
                origin: msg.origin,
                part_index: msg.part_index,
            },
            Name::new("GraphNode"),
            Transform::from_translation(msg.position).with_scale(Vec3::splat(HOOK_SIZE)),
            Pickable::default(),
        ))
        .add_children(&[child_selection_indicator, pattern_child, force_child])
        .observe(on_click_graph_node)
        .id();

    node_lookup.index_to_entity.insert(msg.node_index, entity);
    node_lookup.entity_to_index.insert(entity, msg.node_index);
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

fn report_error(
    error: crochet::errors::Error,
    code_highlighter: &mut Highlighter,
    console_pipe: &ConsolePipe,
) {
    console_pipe.write(format!("Error in pattern: {}", error).as_str());
    // TODO display the error on hover (see poc in code_editor/mod.rs egui::Id::new("token_tooltip"))
    // TODO stop displaying error when text changes
    if let Some(origin) = error.origin() {
        code_highlighter.set(HighlightLayer::RedUnderline, vec![(origin.as_range())]);
    }
}

pub fn build_plushie_from_pattern(
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

    despawn_old_plushie(&mut commands, existing_plushie_entities);
    let (plushie_def, simulated_plushie) =
        match crochet::parse(&msg.acl, HOOK_SIZE, &state.initializer) {
            Ok(x) => x,
            Err(err) => {
                report_error(err, &mut code_editor.highlighter, &pipe);
                commands.remove_resource::<PlushieInSimulation>();
                sync_state.plushie_removed();
                return Ok(());
            }
        };

    let mut node_lookup = NodeLookup::new();

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
                    origin: node.definition.origin.origin, // TODO use the full ActionWithOrigin, create a display mode for it
                    part_index: node.definition.part_index,
                    node_index,
                },
                &mut commands,
                &mut assets,
                &mut materials,
                &display_presets,
                &mut node_lookup,
            )
        })
        .collect();

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
        node_lookup,
    });
    sync_state.plushie_parsed(msg.acl.clone());
    state.active_part = Some(simulated_plushie.parts()[0].name().clone());

    match state.initializer {
        crochet::force_graph::Initializer::RegularCylinder(_) => {
            pipe.write("Built a plushie");
        }
        crochet::force_graph::Initializer::OneByOne => {
            pipe.write("Started building a plushie one by one");
        }
    }

    Ok(())
}

pub fn continue_building_one_by_one(
    mut plushie: ResMut<PlushieInSimulation>,
    mut commands: Commands,
    mut assets: ResMut<PlushieAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    display_presets: Res<DisplayPresets>,
    pipe: Res<ConsolePipe>,
) {
    match plushie.plushie.advance_one_by_one() {
        OneByOneResult::Noop => (),
        OneByOneResult::JustFinished => {
            pipe.write("finished building a plushie one by one");
        }
        OneByOneResult::CreatedNode(new_index) => {
            add_node_to_world(
                &mut plushie,
                new_index,
                &mut commands,
                &mut assets,
                &mut materials,
                &display_presets,
            );
        }
        OneByOneResult::CreatedMagicRing { start, count } => {
            for new_index in start..start + count {
                add_node_to_world(
                    &mut plushie,
                    new_index,
                    &mut commands,
                    &mut assets,
                    &mut materials,
                    &display_presets,
                );
            }
        }
        OneByOneResult::CreatedEdge(node_a, node_b) => {
            let a = plushie
                .node_lookup
                .index_to_entity
                .get(&node_a)
                .expect("index to entity should contain this node durign OBO");
            let b = plushie
                .node_lookup
                .index_to_entity
                .get(&node_b)
                .expect("index to entity should contain this node durign OBO");

            add_link_between(
                *a,
                *b,
                &mut commands,
                &mut assets,
                &mut materials,
                plushie.plushie.nodes()[node_a.max(node_b)].definition.color,
                &display_presets,
            );
        }
    }
}

fn add_node_to_world(
    plushie: &mut PlushieInSimulation,
    new_index: usize,
    commands: &mut Commands,
    assets: &mut PlushieAssets,
    materials: &mut Assets<StandardMaterial>,
    display_presets: &DisplayPresets,
) {
    let new_node = &plushie.plushie.nodes()[new_index];
    let msg = AddGraphNode {
        position: new_node.position,
        color: new_node.definition.color,
        peculiarity: new_node.definition.peculiarity,
        origin: new_node.definition.origin.origin,
        node_index: new_index,
        part_index: new_node.definition.part_index,
    };
    let new_node_entity = add_graph_node(
        &msg,
        commands,
        assets,
        materials,
        &display_presets,
        &mut plushie.node_lookup,
    );

    let new_edges = &plushie.plushie.edges().edges_from_node(new_index);
    for target in new_edges.iter() {
        let a = new_node_entity;
        let b = plushie
            .node_lookup
            .index_to_entity
            .get(target)
            .expect("index to entity should contain lesser-index node");
        add_link_between(
            a,
            *b,
            commands,
            assets,
            materials,
            plushie.plushie.nodes()[new_index].definition.color,
            &display_presets,
        );
    }
}
