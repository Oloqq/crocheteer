// pub mod for_flows;
pub mod legacy;
pub mod params;

use params::Params;

use crate::common::*;

pub trait PlushieTrait: Send + 'static {
    /// Run the simulation until it is considered finished (relaxed)
    fn animate(&mut self);
    /// Run the simulation for one step
    fn step(&mut self, time: f32);
    /// Access parameters of the simulation
    fn params(&mut self) -> &mut Params;

    // JSONs for frontend communication
    fn nodes_to_json(&self) -> JSON;
    fn centroids_to_json(&self) -> JSON;
    fn whole_to_json(&self) -> JSON;

    fn set_point_position(&mut self, i: usize, pos: Point);
    // TODO auto handle setting via params
    fn change_centroid_num(&mut self, num: usize);

    /// As far as I understand, Send and Clone are not compatible
    /// This is a workaround, types implementing the trait can just put Clone::clone() inside
    fn clone(&self) -> Box<dyn PlushieTrait>;
}
