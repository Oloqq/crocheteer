use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod stuffing;
mod systems;

pub use data::{Centroid, LinkForce, NewPosition, Rooted, StuffingForce};
use stuffing::apply_stuffing;

use systems::{
    apply_acceleration, apply_link_forces, reset_acceleration, update_connections_visually,
};

use crate::plushie::animation::systems::move_centroids;

pub struct PlushieAnimationPlugin;

impl Plugin for PlushieAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                reset_acceleration,
                (apply_link_forces, apply_stuffing),
                (
                    apply_acceleration,
                    move_centroids.ambiguous_with(apply_acceleration),
                ),
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
