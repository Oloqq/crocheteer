use bevy::prelude::*;
use crochet::centroid_stuffing;

use crate::plushie::{
    animation::{
        StuffingForce,
        data::{Centroid, NewPosition},
    },
    data::GraphNode,
};

pub fn compute_stuffing_force(
    nodes: Query<(&Transform, &mut StuffingForce), With<GraphNode>>,
    centroids: Query<(&Transform, &mut NewPosition), With<Centroid>>,
) {
    if nodes.iter().len() == 0 || centroids.iter().len() == 0 {
        return;
    }

    let node_positions: Vec<Vec3> = nodes.iter().map(|x| x.0.translation).collect();
    let centroid_positions: Vec<Vec3> = centroids.iter().map(|x| x.0.translation).collect();

    let (node_movement, centroid_new_positions) =
        centroid_stuffing(&node_positions, &centroid_positions);

    for ((_, mut received_force), calculated_stuffing) in
        nodes.into_iter().zip(node_movement.into_iter())
    {
        received_force.0 = calculated_stuffing;
    }
    for ((_, mut new_pos), calculated_new_pos) in centroids
        .into_iter()
        .zip(centroid_new_positions.into_iter())
    {
        new_pos.0 = calculated_new_pos;
    }
}
