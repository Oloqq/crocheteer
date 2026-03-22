use bevy::prelude::*;

use crate::plushie::{
    animation::data::{Link, LinkAssets, LinkForce},
    data::{Dragging, GraphNode, Selected},
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(LinkAssets {
        mesh: meshes.add(Cylinder::new(1.0, 1.0)),
        material: materials.add(Color::srgb(0.2, 0.4, 0.2)),
    });
}

pub fn add_link_between(a: Entity, b: Entity, commands: &mut Commands, assets: &LinkAssets) {
    commands.spawn((
        Link { a, b },
        Mesh3d(assets.mesh.clone()),
        MeshMaterial3d(assets.material.clone()),
        Transform::default(),
    ));
}

pub fn connect(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    selected: Query<Entity, With<Selected>>,
    assets: Res<LinkAssets>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    if selected.iter().len() != 2 {
        info!("select 2 things"); // TODO
        return;
    }

    let vec: Vec<_> = selected.into_iter().collect();
    let a: Entity = vec[0];
    let b: Entity = vec[1];

    commands.entity(a).insert(LinkForce(Vec3::ZERO));
    commands.entity(b).insert(LinkForce(Vec3::ZERO));

    add_link_between(a, b, &mut commands, &assets);
}

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

pub fn reset_acceleration(mut query: Query<&mut LinkForce>) {
    for mut acc in &mut query {
        acc.0 = Vec3::ZERO;
    }
}

pub fn apply_link_forces(
    links: Query<&Link>,
    transforms: Query<&Transform, With<GraphNode>>,
    mut accelerations: Query<&mut LinkForce>,
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
    mut query: Query<(&LinkForce, &mut Transform), (With<GraphNode>, Without<Dragging>)>,
) {
    let dt = time.delta_secs();
    for (acc, mut transform) in &mut query {
        transform.translation += acc.0 * dt * dt;
    }
}
