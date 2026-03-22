use bevy::prelude::*;

use crate::plushie::{
    animation::{
        Centroid, Rooted, StuffingForce,
        data::{LinkForce, NewPosition},
    },
    data::{Dragging, GraphNode, Link},
};

pub fn update_connections_visually(
    mut connections: Query<(&Link, &mut Transform)>,
    node_transforms: Query<&GlobalTransform, With<GraphNode>>,
) {
    for (connection, mut line_transform) in connections.iter_mut() {
        // get_many returns Err if any entity is missing (e.g. a ball was despawned)
        let Ok([transform_a, transform_b]) = node_transforms.get_many([connection.a, connection.b])
        else {
            continue;
        };

        let pos_a = transform_a.translation();
        let pos_b = transform_b.translation();
        let diff = pos_b - pos_a;
        let length = diff.length();

        let thickness = 4e-4;
        let new_trans = Transform {
            translation: ((pos_a + pos_b) / 2.0),
            rotation: Quat::from_rotation_arc(Vec3::Y, diff.normalize()),
            scale: Vec3::new(thickness, length, thickness),
        };

        *line_transform = new_trans;
    }
}

pub fn reset_acceleration(
    mut link_force: Query<&mut LinkForce>,
    mut stuffing_force: Query<&mut StuffingForce>,
    mut displacement: Query<&mut NewPosition>,
) {
    for mut acc in &mut link_force {
        acc.0 = Vec3::ZERO;
    }
    for mut acc in &mut stuffing_force {
        acc.0 = Vec3::ZERO;
    }
    for mut acc in &mut displacement {
        acc.0 = Vec3::ZERO;
    }
}

pub fn apply_link_forces(
    mut accelerations: Query<&mut LinkForce>,
    links: Query<&Link>,
    transforms: Query<&Transform, With<GraphNode>>,
) {
    let desired_length = 5e-3;
    let stiffness = 100.0;
    for link in &links {
        let Ok(src_transform) = transforms.get(link.a) else {
            continue;
        };
        let Ok(tgt_transform) = transforms.get(link.b) else {
            continue;
        };

        let delta = tgt_transform.translation - src_transform.translation;
        let distance = delta.length();
        if distance < f32::EPSILON {
            continue;
        }

        let displacement = distance - desired_length;
        let force = delta.normalize() * displacement * stiffness;

        if let Ok(mut acc) = accelerations.get_mut(link.a) {
            acc.0 += force;
        }
        if let Ok(mut acc) = accelerations.get_mut(link.b) {
            acc.0 -= force;
        }
    }
}

pub fn apply_acceleration(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &LinkForce, &StuffingForce),
        (With<GraphNode>, Without<Dragging>, Without<Rooted>),
    >,
) {
    let dt = time.delta_secs();
    for (mut transform, link_force, stuffing_force) in &mut query {
        transform.translation += (link_force.0 + stuffing_force.0) * dt * dt;
    }
}

pub fn move_centroids(
    mut query: Query<
        (&mut Transform, &NewPosition),
        (With<Centroid>, Without<Dragging>, Without<Rooted>),
    >,
) {
    for (mut transform, new_position) in &mut query {
        transform.translation = new_position.0;
    }
}
