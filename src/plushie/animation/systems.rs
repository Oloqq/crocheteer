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

        let thickness = 1e-4;
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

pub fn apply_forces(
    mut query: Query<
        (&mut Transform, &LinkForce, &StuffingForce),
        (With<GraphNode>, Without<Dragging>, Without<Rooted>),
    >,
) {
    let dt = 1.0 / 64.0;
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
