use bevy::utils::default;
use crocheteer::project::{DisplayMode, Project, SimulationState};

fn main() {
    let project = Project {
        pattern: indoc::indoc! {"
            @centroids = 3,
            @param = yes # comment

            : MR(6) # trailing comment, no round
            : 6 inc (12) # trailing after anchor count
            # expect an error with cap_start in a repetition
            : 12 sc # trailing without anchor count
            # 2: 12 sc, mark(🐸ąęó編みぐるみ) (12) # unicode unfortunately does not work in labels, but is fine in comments
            2: 12 sc, mark(cap_start) (12)
            : BLO, 6 dec (6)
            FO

            goto(cap_start), color(255, 255, 0)
            : FLO, 12 inc (24)
            2: 24 sc (24)
            : 12 dec (12)
            : dec, dec, dec, dec, dec, dec (6)
            FO
        "}
        .into(),

        // TODO initialize in a way that some nodes are already selected
        simulation_config: SimulationState {
            sim_speed: 1.0,
            display_mode: DisplayMode::Forces,
            ..default()
        },
    };
    crocheteer::app(project).run();
}
