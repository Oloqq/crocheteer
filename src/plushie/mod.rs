mod animation;
mod data;
mod mouse_interactions;
mod spawning;
mod systems;

use crate::{
    plushie::{
        animation::{LinkAssets, LinksPlugin, add_link_between},
        mouse_interactions::{deselect_on_empty_press, stop_dragging, update_dragging},
        spawning::{add_graph_node, add_new_nodes},
        systems::{setup_assets, sync_visuals},
    },
    ui::world_input,
};
use bevy::prelude::*;
use data::*;

pub struct PlushiePlugin;

impl Plugin for PlushiePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LinksPlugin);
        app.add_message::<AddNode>();
        app.init_resource::<PressHandled>();
        app.add_systems(Startup, setup_assets);
        app.add_systems(PostStartup, build_a_plushie);
        app.add_systems(
            PreUpdate,
            (
                (deselect_on_empty_press, update_dragging).run_if(world_input),
                stop_dragging,
            ),
        );
        app.add_systems(Update, add_new_nodes);
        app.add_systems(PostUpdate, sync_visuals);
    }
}

fn build_a_plushie(
    mut commands: Commands,
    plushie_assets: Res<PlushieAssets>,
    link_assets: Res<LinkAssets>,
) {
    // let acl = indoc::indoc! {"
    //     @centroids = 3

    //     MR(6)
    //     : 6 inc (12)
    //     3: 12 sc (12)
    //     mark(cap_start)
    //     : BLO, 6 dec (6)
    //     FO

    //     goto(cap_start), color(255, 255, 0)
    //     : FLO, 12 inc (24)
    //     2: 24 sc (24)
    //     : 12 dec (12)
    //     : 6 dec (6)
    //     FO
    // "};

    let acl = indoc::indoc! {"
        MR(6)
    "};
    let (graph_nodes, edges) = crochet::parse(acl);

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
}
