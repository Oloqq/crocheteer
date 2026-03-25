mod animation;
mod data;
mod mouse_interactions;
mod shaders;
mod spawning;
mod systems;

use crate::{
    plushie::{
        animation::PlushieAnimationPlugin,
        mouse_interactions::{deselect_on_empty_press, stop_dragging, update_dragging},
        shaders::{LinkMaterial, learning},
        spawning::build_plushie_from_pattern,
        systems::{setup_assets, sync_visuals},
    },
    ui::world_input,
};
use bevy::prelude::*;
use data::*;

pub use data::BuildPlushieFromPattern;

pub struct PlushiePlugin;

impl Plugin for PlushiePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlushieAnimationPlugin);
        app.add_message::<AddNode>();
        app.add_message::<BuildPlushieFromPattern>();
        app.init_resource::<PressHandled>();
        app.add_plugins(MaterialPlugin::<LinkMaterial>::default());
        app.add_systems(Startup, setup_assets);
        app.add_systems(
            PreUpdate,
            (
                (deselect_on_empty_press, update_dragging).run_if(world_input),
                stop_dragging,
            ),
        );
        app.add_systems(Update, build_plushie_from_pattern);
        app.add_systems(PostUpdate, sync_visuals);

        // {
        //     app.add_systems(PreStartup, learning::setup_material);
        //     app.add_systems(Startup, learning::spawn_entities);
        //     app.add_systems(FixedUpdate, learning::change_prediodically);
        // }
    }
}
