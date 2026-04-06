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
        mouse_interactions::{
            deselect_on_empty_press, highlight_selected_nodes_in_pattern, stop_dragging,
            update_dragging,
        },
        shaders::{LinkMaterial, sync_shader_buffer},
        spawning::{adjust_centroid_number, build_plushie_from_pattern},
        systems::{highlight_selected_nodes_visually, setup_assets},
    },
    ui::{simulation_is_running, world_input},
};
use bevy::prelude::*;
use data::*;

pub use data::BuildPlushieFromPattern;
pub use display_mode::{DisplayMode, SetDisplayMode};

pub struct PlushiePlugin;

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
            (
                build_plushie_from_pattern,
                adjust_centroid_number,
                highlight_selected_nodes_in_pattern,
            )
                .chain(),
        );
        app.add_systems(
            PostUpdate,
            (
                highlight_selected_nodes_visually,
                set_display_mode, // could this be handled with a resource_changed? UiState is dereferenced mutably every frame so probably not right?
                sync_shader_buffer.run_if(simulation_is_running),
            ),
        );

        // {
        //     app.add_systems(PreStartup, learning::setup_material);
        //     app.add_systems(Startup, learning::spawn_entities);
        //     app.add_systems(FixedUpdate, learning::change_prediodically);
        // }
    }
}
