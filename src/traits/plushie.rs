use crate::common::*;

pub trait PlushieTrait {
    fn animate(&mut self);
    fn set_point_position(&mut self, i: usize, pos: Point);
    fn set_centroid_num(&mut self, num: usize);
}
