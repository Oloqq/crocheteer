use bevy::prelude::*;

use crate::plushie::DisplayMode;

#[derive(Resource)]
pub struct SimulationState {
    pub paused: bool,
    pub sim_speed: f64,
    pub force_multiplier: f32,
    pub display_mode: DisplayMode,
    pub centroids: u32,
    // pub nodes_per_centroid: u32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            paused: false,
            sim_speed: 1.0,
            force_multiplier: 1.0,
            display_mode: default(),
            centroids: 0,
            // nodes_per_centroid: 20,
        }
    }
}

pub fn simulation_is_running(ui_state: Res<SimulationState>) -> bool {
    !ui_state.paused
}
