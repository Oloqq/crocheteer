use bevy::prelude::*;

#[derive(Component)]
pub struct Node {}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Dragging {
    pub offset: Vec3,
    pub plane: InfinitePlane3d,
    pub plane_origin: Vec3,
}

#[derive(Resource, Default)]
pub struct PressHandled(pub bool);

#[derive(Resource)]
pub struct PlushieAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub selected_material: Handle<StandardMaterial>,
}

#[derive(Message)]
pub struct AddNode {
    pub position: Vec3,
}
