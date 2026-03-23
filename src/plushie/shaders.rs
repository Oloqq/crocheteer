use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderType;
use bevy::render::storage::ShaderStorageBuffer;
use bevy::shader::ShaderRef;

#[derive(Clone, Copy, ShaderType)]
pub struct LinkInstanceData {
    pub transform: Mat4, // 64 bytes, 16-byte aligned — always put first
    pub force: f32,      // 4 bytes
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct LinkMaterial {
    #[storage(0, read_only)]
    pub instances: Handle<ShaderStorageBuffer>,
    #[uniform(1)]
    pub max_force: f32,
}

impl Material for LinkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/link.wgsl".into() // path relative to your assets/ folder
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/link.wgsl".into()
    }
}

#[derive(Resource)]
pub struct LinkInstanceBuffer(#[allow(dead_code)] pub Handle<ShaderStorageBuffer>);

pub fn setup_link_rendering(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<LinkMaterial>>,
) {
    let instance_data = vec![LinkInstanceData {
        transform: Mat4::IDENTITY,
        force: 1.0,
    }];

    let mut buffer = ShaderStorageBuffer::default();
    buffer.set_data(instance_data.as_slice());
    let buffer_handle = buffers.add(buffer);

    commands.insert_resource(LinkInstanceBuffer(buffer_handle.clone()));

    let material = materials.add(LinkMaterial {
        instances: buffer_handle,
        max_force: 10.0,
    });

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.05, 1.0))),
        MeshMaterial3d(material),
        Transform::default(),
    ));
}
