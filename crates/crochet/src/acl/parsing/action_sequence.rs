use crate::acl::pattern::ActionWithOrigin;

#[derive(Debug)]
pub struct ActionSequence {
    actions: Vec<ActionWithOrigin>,
}

impl ActionSequence {
    pub fn new() -> Self {
        ActionSequence { actions: vec![] }
    }

    pub fn actions(&self) -> &Vec<ActionWithOrigin> {
        &self.actions
    }

    pub fn append_repeated(&mut self, other: ActionSequence, times: u32) {
        self.actions.reserve(other.actions.len() * times as usize);
        for _ in 0..times {
            self.actions.append(&mut other.actions.clone());
        }
    }

    pub fn push(&mut self, action: ActionWithOrigin) {
        self.push_repeated(action, 1);
    }

    pub fn push_repeated(&mut self, action: ActionWithOrigin, times: u32) {
        self.actions.reserve(times as usize);
        for _ in 0..times {
            self.actions.push(action.clone());
        }
    }
}
