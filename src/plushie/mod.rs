mod animation;
mod data;
mod display_mode;
mod mouse_interactions;
mod shaders;
mod spawning;
mod systems;

use crate::{
    plushie::{
        animation::PlushieAnimationPlugin,
        display_mode::{set_display_mode, setup_display_modes},
        mouse_interactions::{deselect_on_empty_press, stop_dragging, update_dragging},
        shaders::{LinkMaterial, sync_shader_buffer},
        spawning::{adjust_centroid_number, build_plushie_from_pattern},
        systems::{setup_assets, sync_visuals_for_selection},
    },
    ui::{simulation_is_running, world_input},
};
use bevy::prelude::*;
use data::*;

pub use data::BuildPlushieFromPattern;
pub use display_mode::{DisplayMode, SetDisplayMode};

pub struct PlushiePlugin {
    pub initial_display_mode: DisplayMode,
}

impl Plugin for PlushiePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlushieAnimationPlugin);
        app.add_message::<AddGraphNode>();
        app.add_message::<BuildPlushieFromPattern>();
        app.add_message::<SetDisplayMode>();
        app.init_resource::<PressHandled>();
        app.add_plugins(MaterialPlugin::<LinkMaterial>::default());
        app.add_systems(Startup, (setup_assets, setup_display_modes).chain());
        app.add_systems(
            PreUpdate,
            (
                (deselect_on_empty_press, update_dragging).run_if(world_input),
                stop_dragging,
            ),
        );
        app.add_systems(
            Update,
            (build_plushie_from_pattern, adjust_centroid_number).chain(),
        );
        app.add_systems(
            PostUpdate,
            (
                sync_visuals_for_selection,
                set_display_mode, // could this be handled with a resource_changed? UiState is dereferenced mutably every frame so probably not right?
                sync_shader_buffer.run_if(simulation_is_running),
            ),
        );

        app.world_mut().write_message(SetDisplayMode {
            mode: self.initial_display_mode,
        });

        // {
        //     app.add_systems(PreStartup, learning::setup_material);
        //     app.add_systems(Startup, learning::spawn_entities);
        //     app.add_systems(FixedUpdate, learning::change_prediodically);
        // }
    }
}
