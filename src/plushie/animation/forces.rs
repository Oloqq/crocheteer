use bevy::prelude::*;

use crate::{
    HOOK_SIZE,
    plushie::{
        animation::{
            LinkForce, Rooted, StuffingForce,
            data::{Centroid, NewPosition, OriginNode, SingleLoopForce},
        },
        data::{Dragging, GraphNode, Link},
    },
    state::simulated_plushie::PlushieInSimulation,
    ui::SimulationState,
};

pub fn compute_stuffing_force(
    nodes: Query<(&Transform, &mut StuffingForce, &GraphNode)>,
    centroids: Query<(&Transform, &mut NewPosition, &Centroid)>,
    plushie: Res<PlushieInSimulation>,
) {
    if nodes.iter().len() == 0 || centroids.iter().len() == 0 {
        return;
    }

    // let node_positions: Vec<Vec3> = nodes.iter().map(|x| x.0.translation).collect();
    // let centroid_positions: Vec<Vec3> = centroids.iter().map(|x| x.0.translation).collect();

    // let (node_movement, centroid_new_positions) = crochet::force_graph::centroid_stuffing::stuff(
    //     &node_positions,
    //     &centroid_positions,
    //     HOOK_SIZE,
    // );

    let node_positions: Vec<(Vec3, usize)> = nodes
        .iter()
        .map(|(transform, _, node)| (transform.translation, node.part_index))
        .collect();
    let centroid_positions: Vec<(Vec3, usize)> = centroids
        .iter()
        .map(|(transform, _, centroid)| (transform.translation, centroid.part))
        .collect();

    let (node_movement, centroid_new_positions) =
        crochet::force_graph::centroid_stuffing::per_part::stuff(
            &node_positions,
            &centroid_positions,
            HOOK_SIZE,
            plushie.plushie.pattern.parts.len(),
        );

    for ((_, mut received_force, _), calculated_stuffing) in
        nodes.into_iter().zip(node_movement.into_iter())
    {
        received_force.0 = calculated_stuffing;
    }
    for ((_, mut new_pos, _), calculated_new_pos) in centroids
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
    let desired_stitch_distance = HOOK_SIZE;
    for mut link in links {
        let Ok(src_transform) = transforms.get(link.node_a) else {
            continue;
        };
        let Ok(tgt_transform) = transforms.get(link.node_b) else {
            continue;
        };

        let diff = &src_transform.translation - &tgt_transform.translation;
        let magnitude = crochet::force_graph::link_force::link_force_magnitude(
            diff.length(),
            desired_stitch_distance,
        );
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

pub fn compute_single_loop_force(
    nodes: Query<(&Transform, &GraphNode, &mut SingleLoopForce)>,
    state: Res<SimulationState>,
) {
    if nodes.iter().len() == 0 {
        return;
    }

    let input: Vec<_> = nodes
        .iter()
        .map(|node| (node.0.translation, node.1.peculiarity))
        .collect();

    let normals = crochet::force_graph::single_loop::find_normals(&input);

    for ((_, _, mut received_force), calculated_normal) in
        nodes.into_iter().zip(normals.into_iter())
    {
        received_force.0 = calculated_normal * state.single_loop_force;
    }
}

pub fn apply_forces(
    mut query: Query<
        (&mut Transform, &LinkForce, &StuffingForce, &SingleLoopForce),
        (With<GraphNode>, Without<Dragging>, Without<Rooted>), // maybe the dragging system should be inserting the Rooted component instead of double Without?
    >,
    params: Res<SimulationState>,
    origin_node: Option<Single<Entity, With<OriginNode>>>, // single can't work with multipart
) {
    let force_multiplier = 0.0003 * params.force_multiplier;
    let origin_node_displacement = origin_node
        .and_then(|origin_entity| query.get(*origin_entity).ok())
        .map(|(_, link_force, stuffing_force, _single_loop)| {
            displacement(link_force.0, stuffing_force.0, Vec3::ZERO, force_multiplier)
        })
        .unwrap_or(Vec3::ZERO);

    for (mut transform, link_force, stuffing_force, single_loop_force) in &mut query {
        transform.translation += displacement(
            link_force.0,
            stuffing_force.0,
            single_loop_force.0,
            force_multiplier,
        ) - origin_node_displacement;
    }
}

fn displacement(
    link_force: Vec3,
    stuffing_force: Vec3,
    single_loop_force: Vec3,
    force_multiplier: f32,
) -> Vec3 {
    (link_force + stuffing_force + single_loop_force) * force_multiplier
}
