use bevy::prelude::*;

use crate::plushie::data::{GraphNode, Link};

pub fn update_connections_visually(
    mut connections: Query<(&Link, &mut Transform)>,
    node_transforms: Query<&GlobalTransform, With<GraphNode>>,
) {
    for (connection, mut line_transform) in connections.iter_mut() {
        // get_many returns Err if any entity is missing (e.g. a ball was despawned)
        let Ok([transform_a, transform_b]) =
            node_transforms.get_many([connection.node_a, connection.node_b])
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
