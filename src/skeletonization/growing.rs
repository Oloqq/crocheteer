use super::initial_cross_sections::CrossSection;
use crate::common::*;

pub struct Part {
    pub sections: Vec<CrossSection>,
}

pub fn grow(cloud: &Vec<Point>, cross_sections: Vec<CrossSection>) -> Vec<Part> {
    todo!();
}
