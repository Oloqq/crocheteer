use bevy::prelude::*;
use crochet::{centroid_stuffing, link_force_magnitude};

use crate::plushie::{
    animation::{
        LinkForce, StuffingForce,
        data::{Centroid, NewPosition},
    },
    data::{GraphNode, Link},
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

pub fn compute_link_forces(
    mut accelerations: Query<&mut LinkForce>,
    links: Query<&mut Link>,
    transforms: Query<&Transform, With<GraphNode>>,
) {
    let desired_stitch_distance = 5e-4;
    for mut link in links {
        let Ok(src_transform) = transforms.get(link.node_a) else {
            continue;
        };
        let Ok(tgt_transform) = transforms.get(link.node_b) else {
            continue;
        };

        let diff = &src_transform.translation - &tgt_transform.translation;
        let magnitude = link_force_magnitude(diff.length(), desired_stitch_distance);
        link.tension = magnitude;
        let force: Vec3 = -diff.normalize() * magnitude;

        if let Ok(mut acc) = accelerations.get_mut(link.node_a) {
            acc.0 += force;
        }
        if let Ok(mut acc) = accelerations.get_mut(link.node_b) {
            acc.0 -= force;
        }
    }
}
