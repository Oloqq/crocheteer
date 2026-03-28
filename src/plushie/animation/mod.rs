use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod forces;
mod systems;

pub use data::{Centroid, LinkForce, NewPosition, OriginNode, Rooted, StuffingForce};

use systems::{reset_acceleration, update_connections_visually};

use crate::{
    plushie::animation::{
        forces::{apply_forces, compute_link_forces, compute_stuffing_force},
        systems::move_centroids,
    },
    ui::simulation_is_running,
};

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
                .chain()
                .run_if(simulation_is_running),
        );
        app.add_systems(
            PostUpdate,
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
