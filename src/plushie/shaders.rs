use bevy::mesh::MeshTag;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderType;
use bevy::render::storage::ShaderStorageBuffer;
use bevy::shader::ShaderRef;

#[derive(Clone, Copy, ShaderType, Debug)]
pub struct LinkInstanceData {
    pub force: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct LinkMaterial {
    #[storage(0, read_only)]
    pub instances: Handle<ShaderStorageBuffer>,
}

impl Material for LinkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/link.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/link.wgsl".into()
    }
}

#[allow(dead_code)] // to be removed
pub mod learning {

    use super::*;

    #[derive(Component)]
    pub struct TestLink(f32);

    #[derive(Resource)]
    pub struct LinkRenderHandles {
        material: Handle<LinkMaterial>,
    }

    pub fn setup_material(
        mut commands: Commands,
        mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
        mut materials: ResMut<Assets<LinkMaterial>>,
    ) {
        let mut buffer = ShaderStorageBuffer::default();
        let initial_data = vec![
            LinkInstanceData { force: 0.5 },
            LinkInstanceData { force: 0.0 },
        ];
        buffer.set_data(initial_data.as_slice());
        let buffer_handle = buffers.add(buffer);

        let material = materials.add(LinkMaterial {
            instances: buffer_handle.clone(),
        });

        commands.insert_resource(LinkRenderHandles {
            material: material.clone(),
        });
    }

    pub fn spawn_entities(
        mut commands: Commands,
        handles: Res<LinkRenderHandles>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(1.0, 1.0))),
            MeshMaterial3d(handles.material.clone()),
            // index into `instances` storage buffer
            // if out of bounds, shader silently uses default value for the data
            MeshTag(0u32),
            TestLink(1.0),
            Transform::default().with_scale(Vec3::splat(0.01)),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(1.0, 1.0))),
            MeshMaterial3d(handles.material.clone()),
            TestLink(0.5),
            MeshTag(1u32), // index into your `instances` storage buffer
            Transform::default()
                .with_scale(Vec3::splat(0.01))
                .with_translation(Vec3::new(0.02, 0.0, 0.0)),
        ));
    }

    pub fn change_prediodically(
        mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
        mut materials: ResMut<Assets<LinkMaterial>>,
        mut y: Local<f32>,
        handles: Res<LinkRenderHandles>,
    ) {
        *y += 0.01;

        let instance_data = vec![
            LinkInstanceData { force: 0.5 },
            LinkInstanceData { force: y.min(1.0) },
        ];

        if *y > 1.0 {
            *y = 0.0;
        }

        // get_mut is mandatory because it emits an event that tells bevy to reupload to GPU
        // it would be so much easier to prepare a discrete number of materials in advance
        // and just swap the handles depending on force
        let mat = materials.get_mut(&handles.material).unwrap();
        let buffer = buffers.get_mut(&mat.instances).unwrap();
        buffer.set_data(instance_data.as_slice());
    }
}
