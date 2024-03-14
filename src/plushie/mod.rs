// pub mod for_flows;
pub mod legacy;
pub mod params;

// TODO centroids = 0 === no stuffing, remove none variant
use legacy::{animation::centroid::Centroids, Stuffing};
use params::Params;

use crate::common::*;

pub trait PlushieTrait: Send + 'static {
    /// Run the simulation until it is considered finished (relaxed)
    fn animate(&mut self);
    /// Run the simulation for one step
    fn step(&mut self, time: f32);
    /// Access parameters of the simulation
    fn params(&mut self) -> &mut Params;

    fn set_point_position(&mut self, i: usize, pos: Point);
    fn set_centroid_num(&mut self, num: usize);
    fn set_stuffing(&mut self, stuffing: Stuffing);

    fn get_points_vec(&self) -> &Vec<Point>;
    fn get_centroids(&self) -> &Centroids;
    fn stuffing(&self) -> &Stuffing;

    // workarounds
    //
    /// As far as I understand, Send and Clone are not compatible
    /// This is a workaround, types implementing the trait can just put Clone::clone() inside
    fn clone(&self) -> Box<dyn PlushieTrait>;
    /// Same story as clone()
    fn serialize(&self) -> serde_json::Value;
}
