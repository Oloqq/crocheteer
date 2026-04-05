use crate::{ColorRgb, hook::Hook};

#[derive(Clone, Debug)]
pub struct Node {
    pub color: ColorRgb,
    pub peculiarity: Option<Peculiarity>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Peculiarity {
    Locked,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
}

pub type PointsOnPushPlane = (usize, usize, usize);

impl Hook {
    pub fn add_node(&mut self, peculiarity: Option<Peculiarity>) {
        self.nodes.push(Node {
            color: self.color,
            peculiarity,
        });
    }
}
