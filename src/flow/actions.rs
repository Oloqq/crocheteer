type Label = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum Action {
    Sc,
    Inc,
    Dec,
    Ch(usize),
    Attach(usize),
    Reverse,
    FLO,
    BLO,
    Both,
    Goto(Label),
    Mark(Label),
    MR(usize),
    FO,
}

impl Action {
    pub fn is_starter(&self) -> bool {
        if matches!(self, Action::Ch(_)) {
            unimplemented!()
        }
        matches!(self, Action::MR(_))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
