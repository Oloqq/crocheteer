use bevy::prelude::*;

// TODO
// goals:
// - ✅stop highlights when out of sync
// - ✅changing something, than ctrl+z is considered in sync (obviously, returning to the previous state by hand too)
// - changing things that do not change the action Flow is considered in sync (e.g. changing comments, replacing "[sc] x 3" with "3 sc" or "sc, sc, sc" ). This needs to rerun the hook, but without restarting the simulation

#[derive(Resource)]
pub struct EditorSimulationSync {
    pub acl_in_simulation: Option<String>,
    // pub pattern_in_simulation: Option<crochet::Pattern>,
    pub in_sync: bool,
}

impl EditorSimulationSync {
    pub fn new() -> Self {
        Self {
            acl_in_simulation: None,
            // pattern_in_simulation: None,
            in_sync: true,
        }
    }

    pub fn editor_changed(&mut self, new_acl: &str) {
        self.in_sync = match &self.acl_in_simulation {
            Some(prev) => prev == new_acl,
            None => false,
        };
        // self.in_sync = match crochet::acl_to_pattern(&new_acl) {
        //     Ok(new_pattern) => {
        //         // new_pattern;
        //         //
        //     }
        //     Err(_) => false,
        // };
    }

    pub fn plushie_parsed(&mut self, acl: String) {
        self.acl_in_simulation = Some(acl);
        // self.pattern_in_simulation = Some(pattern);
        self.in_sync = true;
    }
}
