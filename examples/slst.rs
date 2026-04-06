use bevy::utils::default;
use crocheteer::project::{DisplayMode, Project, SimulationState};

fn main() {
    let project = Project {
        pattern: indoc::indoc! {"
            MR(6)
            : 6 inc (12)
            3: 12 sc (12)
            : 11 sc, slst
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            centroids: 3,
            display_mode: DisplayMode::Forces,
            ..default()
        },
    };
    crocheteer::app(project).run();
}
