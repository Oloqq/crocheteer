use bevy::prelude::*;

#[derive(Component, Default)]
pub struct LinkForce(pub Vec3);

#[derive(Resource)]
pub struct LinkAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Link {
    pub a: Entity,
    pub b: Entity,
}
