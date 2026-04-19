use bevy::prelude::*;

use crate::{
    HOOK_SIZE,
    plushie::{
        animation::{Centroid, NewPosition},
        data::PlushieAssets,
    },
    state::simulated_plushie::PlushieInSimulation,
};

pub fn adjust_centroid_number(
    mut commands: Commands,
    existing_centroids: Query<(Entity, &Centroid)>,
    plushie: Res<PlushieInSimulation>,
    assets: Res<PlushieAssets>,
) {
    for (i, part) in plushie.plushie.pattern.parts.iter().enumerate() {
        let new_count = part.parameters.centroids;
        let centroids_of_this_part = existing_centroids.iter().filter(|(_, c)| c.part == i);
        let existing = centroids_of_this_part.clone().count();
        if new_count > existing {
            for _ in 0..(new_count - existing) {
                add_centroid(&mut commands, &assets, i);
            }
        } else {
            for (entity, _) in centroids_of_this_part.skip(new_count) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn add_centroid(commands: &mut Commands, assets: &PlushieAssets, part: usize) {
    commands.spawn((
        Centroid { part },
        Name::new("Centroid"),
        NewPosition::default(),
        Mesh3d(assets.node_mesh.clone()),
        MeshMaterial3d(assets.centroid_material.clone()),
        Transform::from_scale(Vec3::splat(HOOK_SIZE)), // does not necessarily have to be equal to hook size, purely visual preference
        Pickable::default(),
    ));
}
