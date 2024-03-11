type Label = usize;

#[derive(Debug)]
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
