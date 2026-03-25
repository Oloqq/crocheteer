use crate::plushie::shaders::{LinkInstanceData, LinkMaterial};

use super::data::GraphNode;
use super::data::*;
use bevy::mesh::MeshTag;
use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;

pub fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut link_shader_materials: ResMut<Assets<LinkMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let assets = PlushieAssets {
        stitch_mesh: meshes.add(Sphere::new(1.0)),
        stitch_material: materials.add(Color::srgb(1.0, 0.4, 0.4)),
        selected_material: materials.add(Color::srgb(1.0, 1.0, 1.0)), // TODO https://github.com/komadori/bevy_mod_outline
        link_mesh: meshes.add(Cylinder::new(1.0, 1.0)),
        link_material: materials.add(Color::srgb(0.2, 0.4, 0.2)),
        force_responding_material: link_shader_materials.add(LinkMaterial {
            instances: buffers.add(ShaderStorageBuffer::default()),
        }),
    };
    commands.insert_resource(assets);
}

pub fn sync_visuals(
    mut commands: Commands,
    mut removed: RemovedComponents<Selected>,
    mut shader_materials: ResMut<Assets<LinkMaterial>>,
    mut shader_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    links: Query<(Entity, &Link)>,
    assets: Res<PlushieAssets>,
    added: Query<Entity, (With<GraphNode>, With<Selected>, Added<Selected>)>,
) {
    for entity in &added {
        commands
            .entity(entity)
            .insert(MeshMaterial3d(assets.selected_material.clone()));
    }
    for entity in removed.read() {
        commands
            .entity(entity)
            .insert(MeshMaterial3d(assets.stitch_material.clone()));
    }

    let material = shader_materials
        .get_mut(&assets.force_responding_material)
        .unwrap();
    let buffer = shader_buffers.get_mut(&material.instances).unwrap();
    let instance_data: Vec<_> = links
        .iter()
        .enumerate()
        .map(|(i, (entity, link))| {
            commands.entity(entity).insert(MeshTag(i as u32));
            LinkInstanceData {
                force: link.tension,
            }
        })
        .collect();
    buffer.set_data(instance_data);
}
