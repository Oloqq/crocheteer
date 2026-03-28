pub use crate::plushie::DisplayMode;
// TODO move out of ui namespace
pub use crate::ui::SimulationState;

pub struct Project {
    pub pattern: String,
    pub simulation_config: SimulationState,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            pattern: "MR(6)".into(),
            simulation_config: Default::default(),
        }
    }
}

pub mod startup {
    use super::*;
    use bevy::prelude::*;

    use crate::{FIXED_UPDATE_BASE_HZ, plushie::SetDisplayMode};

    pub fn apply_settings(app: &mut App, state: &SimulationState) {
        let timestep = Time::<Fixed>::from_hz(FIXED_UPDATE_BASE_HZ * state.sim_speed);
        app.insert_resource(timestep);
        app.world_mut().write_message(SetDisplayMode {
            mode: state.display_mode,
        });
    }
}
