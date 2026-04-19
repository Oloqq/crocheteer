use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    HOOK_SIZE,
    plushie::{
        animation::data::Centroid,
        data::{Link, PlushieAssets},
    },
    state::simulated_plushie::PlushieInSimulation,
    ui::SimulationState,
};

pub fn simulation_step(
    mut commands: Commands,
    mut plushie: ResMut<PlushieInSimulation>,
    params: Res<SimulationState>,
    mut transforms: Query<&mut Transform>,
    existing_centroids: Query<Entity, With<Centroid>>,
    assets: Res<PlushieAssets>,
    links: Query<&mut Link>,
) {
    plushie.plushie.step(
        &crochet::force_graph::simulated_plushie::step::SimulationParams {
            force_multiplier: 0.0003 * params.force_multiplier,
            single_loop_force: params.single_loop_force,
        },
    );

    // these blocks in braces could be their own systems

    {
        // copy node positions
        for (i, node) in plushie.plushie.nodes().iter().enumerate() {
            let Some(entity) = plushie.node_lookup.index_to_entity.get(&i) else {
                continue;
            };
            match transforms.get_mut(*entity) {
                Ok(mut trans) => trans.translation = node.position,
                Err(_) => {
                    warn!("missing node");
                    continue;
                }
            }
        }
    }

    {
        // copy centroids
        let centroids_positions = plushie.plushie.get_centroids();
        let mut positions_iter = centroids_positions.iter();
        let mut centroid_entities: Vec<Entity> = existing_centroids.iter().collect();
        let need_to_add = centroids_positions
            .len()
            .saturating_sub(centroid_entities.len());
        if centroids_positions.len() > centroid_entities.len() {
            for _ in 0..need_to_add {
                centroid_entities.push(add_centroid(
                    &mut commands,
                    &assets,
                    *positions_iter.next().unwrap(),
                ));
            }
        } else {
            while centroid_entities.len() > centroids_positions.len() {
                commands.entity(centroid_entities.pop().unwrap()).despawn();
            }
        }
        assert_eq!(centroids_positions.len(), centroid_entities.len());
        // zip trims the newly added centroid entity
        for (centroid, entity) in positions_iter.zip(centroid_entities) {
            match transforms.get_mut(entity) {
                Ok(mut trans) => trans.translation = *centroid,
                Err(_) => {
                    warn!("missing centroid");
                    continue;
                }
            }
        }
    }

    {
        // update link tension
        let mut lookup: HashMap<(Entity, Entity), f32> = HashMap::new();
        for (index_larger, values) in plushie.plushie.get_tensions().iter().enumerate() {
            for (edge_index, tension) in values.iter().enumerate() {
                let a = plushie.node_lookup.index_to_entity.get(&index_larger);
                let b = plushie
                    .node_lookup
                    .index_to_entity
                    .get(&plushie.plushie.edges().data()[index_larger][edge_index]);
                match (a, b) {
                    (Some(a), Some(b)) => {
                        lookup.insert((*a, *b), *tension);
                        lookup.insert((*b, *a), *tension);
                    }
                    _ => {
                        warn!("missing node in link lookup");
                    }
                }
            }
        }
        for mut link in links {
            match lookup.get(&(link.node_a, link.node_b)) {
                Some(tension) => link.tension = *tension,
                None => {
                    warn!("missing entry in lookup table")
                }
            }
        }
    }
}

fn add_centroid(commands: &mut Commands, assets: &PlushieAssets, translation: Vec3) -> Entity {
    let centroid_visual_radius = HOOK_SIZE;
    commands
        .spawn((
            Centroid,
            Name::new("Centroid"),
            Mesh3d(assets.node_mesh.clone()),
            MeshMaterial3d(assets.centroid_material.clone()),
            Transform::from_scale(Vec3::splat(centroid_visual_radius))
                .with_translation(translation),
            Pickable::default(),
        ))
        .id()
}

// pub fn apply_forces(
//     mut query: Query<(&mut Transform), (With<GraphNode>, Without<Rooted>)>,
//     params: Res<SimulationState>,
//     origin_node: Option<Single<Entity, With<OriginNode>>>, // single can't work with multipart
// ) {
//     let force_multiplier = 0.0003 * params.force_multiplier;
//     let origin_node_displacement = origin_node
//         .and_then(|origin_entity| query.get(*origin_entity).ok())
//         .map(|(_, link_force, stuffing_force, _single_loop)| {
//             displacement(link_force.0, stuffing_force.0, Vec3::ZERO, force_multiplier)
//         })
//         .unwrap_or(Vec3::ZERO);

//     for (mut transform, link_force, stuffing_force, single_loop_force) in &mut query {
//         transform.translation += displacement(
//             link_force.0,
//             stuffing_force.0,
//             single_loop_force.0,
//             force_multiplier,
//         ) - origin_node_displacement;
//     }
// }
