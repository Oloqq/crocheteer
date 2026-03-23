use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod stuffing;
mod systems;

pub use data::{Centroid, LinkForce, NewPosition, Rooted, StuffingForce};
use stuffing::compute_stuffing_force;

use systems::{apply_forces, compute_link_forces, reset_acceleration, update_connections_visually};

use crate::plushie::animation::systems::move_centroids;

pub struct PlushieAnimationPlugin;

impl Plugin for PlushieAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                reset_acceleration,
                (compute_link_forces, compute_stuffing_force),
                (apply_forces, move_centroids.ambiguous_with(apply_forces)),
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
