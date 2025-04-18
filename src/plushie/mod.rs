/// Plushie provides functionality to animate a set of points
/// and visualize display the process
pub mod examples;
pub mod params;

mod animated;
mod pointcloud;

pub use animated::{perf, Plushie};
pub use params::Params;

use crate::common::*;

pub trait PlushieTrait: Send + 'static {
    /// Run the simulation for one step
    fn step(&mut self, time: f32);
    /// Access parameters of the simulation
    fn params(&self) -> &Params;
    fn params_mut(&mut self) -> &mut Params;
    /// Overwrite params
    fn set_params(&mut self, params: Params);

    fn get_points(&self) -> &Vec<Point>;

    // JSONs for frontend communication
    fn nodes_to_json(&self) -> JSON;
    fn centroids_to_json(&self) -> JSON;
    fn init_data(&self) -> JSON;

    fn set_point_position(&mut self, i: usize, pos: Point);

    /// As far as I understand, Send and Clone are not compatible
    /// This is a workaround, types implementing the trait can just put Clone::clone() inside
    fn clonebox(&self) -> Box<dyn PlushieTrait>;

    fn as_animated(&self) -> Option<&animated::Plushie>;
}
