use bevy::utils::default;
use crocheteer::project::{DisplayMode, Project, SimulationState};

fn main() {
    let project = Project {
        pattern: indoc::indoc! {"
            : MR(6)
            : 6 inc (12)
            3: 12 sc (12)
            mark(cap_start)
            : BLO, 6 dec (6)
            FO

            goto(cap_start), color(255, 255, 0)
            : FLO, 12 inc (24)
            2: 24 sc (24)
            : 12 dec (12)
            : 6 dec (6)
            FO
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
