use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod forces;
mod systems;

pub use data::{Centroid, LinkForce, Rooted, SingleLoopForce, StuffingForce};

use systems::update_connections_visually;

use crate::{plushie::animation::forces::simulation_step, ui::simulation_is_running};

pub struct PlushieAnimationPlugin;

impl Plugin for PlushieAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, simulation_step.run_if(simulation_is_running));
        app.add_systems(
            PostUpdate,
            // this is in update to respond to user dragging nodes
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
