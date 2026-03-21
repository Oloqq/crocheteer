use super::data::Node;
use super::data::*;
use bevy::prelude::*;

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let assets = PlushieAssets {
        mesh: meshes.add(Sphere::new(1.0)),
        material: materials.add(Color::srgb(1.0, 0.4, 0.4)),
        selected_material: materials.add(Color::srgb(0.7, 0.7, 0.7)),
    };
    commands.insert_resource(assets);
}

pub fn add_new_nodes(
    mut commands: Commands,
    mut msgr: MessageReader<AddNode>,
    assets: Res<PlushieAssets>,
) {
    let radius = 0.001;
    for msg in msgr.read() {
        commands.spawn((
            Node {},
            // LinkForce(Vec3::ZERO),
            Name::new("Ball"),
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.material.clone()),
            Transform::from_translation(msg.position).with_scale(Vec3::splat(radius)),
            Pickable::default(),
        ));
        // .observe(on_click_ball);
    }
}
