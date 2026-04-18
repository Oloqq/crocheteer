use bevy::prelude::*;

pub use crate::plushie::DisplayMode;
use crate::ui::action_item::UiActionItem;

#[derive(Resource)]
pub struct SimulationState {
    pub paused: bool,
    pub sim_speed: f64,
    pub force_multiplier: f32,
    pub display_mode: DisplayMode,
    pub centroids: u32,
    pub single_loop_force: f32,
    pub initializer: crochet::force_graph::Initializer,
    pub active_part: Option<String>,
    pub action_items: Vec<UiActionItem>,
}

impl Default for SimulationState {
    fn default() -> Self {
        let s: String = "aaa".into();
        println!("{}, {:?}", s, s);
        Self {
            paused: false,
            sim_speed: 1.0,
            force_multiplier: 1.0,
            display_mode: default(),
            centroids: 0,
            single_loop_force: 0.2,
            initializer: crochet::force_graph::Initializer::RegularCylinder(12),
            active_part: None,
            action_items: vec![],
        }
    }
}

pub fn simulation_is_running(ui_state: Res<SimulationState>) -> bool {
    !ui_state.paused
}
