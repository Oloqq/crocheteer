mod animation;
mod data;
mod mouse_interactions;
mod spawning;
mod systems;

use crate::{
    plushie::{
        animation::LinksPlugin,
        mouse_interactions::{deselect_on_empty_press, stop_dragging, update_dragging},
        spawning::{add_new_nodes, build_plushie_from_pattern},
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
        app.add_plugins(LinksPlugin);
        app.add_message::<AddNode>();
        app.add_message::<BuildPlushieFromPattern>();
        app.init_resource::<PressHandled>();
        app.add_systems(Startup, setup_assets);
        app.add_systems(
            PreUpdate,
            (
                (deselect_on_empty_press, update_dragging).run_if(world_input),
                stop_dragging,
            ),
        );
        app.add_systems(Update, (build_plushie_from_pattern, add_new_nodes).chain());
        app.add_systems(PostUpdate, sync_visuals);
    }
}
