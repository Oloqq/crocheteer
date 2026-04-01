use super::Action;

#[derive(Debug)]
pub struct ActionSequence {
    actions: Vec<Action>,
}

impl ActionSequence {
    pub fn new() -> Self {
        ActionSequence { actions: vec![] }
    }

    pub fn actions(&self) -> &Vec<Action> {
        &self.actions
    }

    pub fn append(&mut self, other: ActionSequence) {
        self.append_repeated(other, 1);
    }

    pub fn append_repeated(&mut self, other: ActionSequence, times: u32) {
        self.actions.reserve(other.actions.len() * times as usize);
        for _ in 0..times {
            self.actions.append(&mut other.actions.clone());
        }
    }

    pub fn push(&mut self, action: Action) {
        self.push_repeated(action, 1);
    }

    pub fn push_repeated(&mut self, action: Action, times: u32) {
        self.actions.reserve(times as usize);
        for _ in 0..times {
            self.actions.push(action.clone());
        }
    }
}
