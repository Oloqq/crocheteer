use crate::{ColorRgb, acl::Origin, hook::Hook};

#[derive(Clone, Debug)]
pub struct Node {
    pub color: ColorRgb,
    pub peculiarity: Option<Peculiarity>,
    /// The location in the pattern that caused creation of this node.
    pub origin: Option<Origin>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Peculiarity {
    Locked,
    Tip,
    BLO(PointsOnPushPlane),
    FLO(PointsOnPushPlane),
}

pub type PointsOnPushPlane = (usize, usize, usize);

impl Hook {
    pub fn add_node(&mut self, peculiarity: Option<Peculiarity>, origin: Option<Origin>) {
        self.nodes.push(Node {
            color: self.color,
            peculiarity,
            origin,
        });
    }
}
