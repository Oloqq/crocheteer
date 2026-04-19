use bevy::{prelude::*, transform::plugins::TransformSystems};

mod data;
mod forces;
mod systems;

pub use data::Centroid;

use systems::update_connections_visually;

use crate::{
    plushie::{animation::forces::simulation_step, spawning::continue_building_one_by_one},
    ui::simulation_is_running,
};

pub struct PlushieAnimationPlugin;

impl Plugin for PlushieAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (continue_building_one_by_one, simulation_step)
                .chain()
                .run_if(simulation_is_running),
        );
        app.add_systems(
            PostUpdate,
            // this is in update to respond to user dragging nodes
            update_connections_visually.after(TransformSystems::Propagate),
        );
    }
}
