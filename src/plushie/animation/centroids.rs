use bevy::prelude::*;

use crate::{
    HOOK_SIZE,
    plushie::{
        animation::{Centroid, NewPosition},
        data::PlushieAssets,
    },
    ui::SimulationState,
};

pub fn adjust_centroid_number(
    mut commands: Commands,
    state: Res<SimulationState>,
    existing_centroids: Query<Entity, With<Centroid>>,
    assets: Res<PlushieAssets>,
) {
    let new_count = state.centroids as usize;
    let existing = existing_centroids.iter().len();
    if new_count > existing {
        for _ in 0..(new_count - existing) {
            add_centroid(&mut commands, &assets);
        }
    } else {
        for entity in existing_centroids.iter().skip(new_count) {
            commands.entity(entity).despawn();
        }
    }
}

fn add_centroid(commands: &mut Commands, assets: &PlushieAssets) {
    commands.spawn((
        Centroid,
        Name::new("Centroid"),
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.centroid_material.clone()),
        Transform::from_scale(Vec3::splat(HOOK_SIZE)), // does not necessarily have to be equal to hook size, purely visual preference
        Pickable::default(),
    ));
}
