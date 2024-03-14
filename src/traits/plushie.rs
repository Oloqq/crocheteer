use crate::{
    common::*,
    plushie::{self, animation::centroid::Centroids, params::Params, Stuffing},
};

pub trait PlushieTrait: Send + 'static {
    fn to_plushie_1(self) -> plushie::Plushie;
    fn animate(&mut self);
    fn set_point_position(&mut self, i: usize, pos: Point);
    fn set_centroid_num(&mut self, num: usize);
    fn get_points_vec(&self) -> &Vec<Point>;
    fn get_centroids(&self) -> &Centroids;
    fn step(&mut self, time: f32);
    fn params(&mut self) -> &mut Params;
    fn stuffing(&self) -> &Stuffing;
    fn set_stuffing(&mut self, stuffing: Stuffing);
    /// As far as I understand, Send and Clone are not compatible
    /// This is a workaround, types implementing the trait can just put Clone::clone() inside
    fn clone(&self) -> Box<dyn PlushieTrait>;
    /// Same story as clone()
    fn serialize(&self) -> serde_json::Value;
}
