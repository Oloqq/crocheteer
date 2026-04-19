use bevy::{prelude::*, transform::plugins::TransformSystems};

mod centroids;
mod data;
mod forces;
mod systems;

pub use data::{
    Centroid, LinkForce, NewPosition, OriginNode, Rooted, SingleLoopForce, StuffingForce,
};

use systems::{reset_acceleration, update_connections_visually};

use crate::{
    plushie::animation::{
        centroids::adjust_centroid_number,
        forces::{apply_forces, compute_single_loop_force, simulation_step},
        systems::move_centroids,
    },
    ui::simulation_is_running,
};

pub struct PlushieAnimationPlugin;

impl Plugin for PlushieAnimationPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     FixedUpdate,
        //     (
        //         (reset_acceleration, adjust_centroid_number),
        //         simulation_step,
        //         (
        //             compute_link_forces,
        //             compute_stuffing_force,
        //             compute_single_loop_force,
        //         ),
        //         (apply_forces, move_centroids.ambiguous_with(apply_forces)),
        //     )
        //         .chain()
        //         .run_if(simulation_is_running),
        // );
        app.add_systems(FixedUpdate, simulation_step.run_if(simulation_is_running));
        app.add_systems(
            PostUpdate,
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
