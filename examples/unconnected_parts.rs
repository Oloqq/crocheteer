use bevy::utils::default;
use crocheteer::project::{DisplayMode, Project, SimulationState};

fn main() {
    let project = Project {
        pattern: indoc::indoc! {"
            == Stem ==
            @centroids = 1

            : MR(6)
            : 6 inc (12)
            2: 12 sc (12)
            : BLO, 6 dec (6)
            FO

            == Cap ==
            @centroids = 2

            color(255, 255, 0)
            : MR(6)
            : 6 inc (12)
            : [sc, inc] x 6 (18)
            : [2 sc, inc] x 6 (24)
            : 12 dec (12)
            : 6 dec (6)
            FO
        "}
        .into(),
        simulation_config: SimulationState {
            sim_speed: 1.0,
            centroids: 0,
            single_loop_force: 0.0,
            display_mode: DisplayMode::Forces,
            ..default()
        },
    };
    crocheteer::app(project).run();
}
