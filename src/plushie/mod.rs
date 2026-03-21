use bevy::prelude::*;

use crate::plushie::systems::{add_new_nodes, setup_assets};

pub struct PlushiePlugin;

mod data;
mod systems;

use data::*;

impl Plugin for PlushiePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AddNode>();
        app.add_systems(Startup, setup_assets);
        app.add_systems(Startup, build_a_plushie);
        app.add_systems(Update, add_new_nodes);
    }
}

fn build_a_plushie(mut msgw: MessageWriter<AddNode>) {
    let acl = indoc::indoc! {"
        @centroids = 3

        MR(6)
        : 6 inc (12)
        3: 12 sc (12)
        mark(cap_start)
        : BLO, 6 dec (6)
        FO

        goto(cap_start), color(255, 255, 0)
        : FLO, 12 inc (24)
        2: 24 sc (24)
        : 12 dec (12)
        : 6 dec (6)
        FO
    "};
    let graph_nodes: Vec<Vec3> = crochet::parse_into_points(acl);

    for node in graph_nodes {
        msgw.write(AddNode { position: node });
    }
}
