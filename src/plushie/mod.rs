pub mod examples;
pub mod params;

mod for_flows;
mod legacy;

pub use for_flows::Plushie;
pub use legacy::Plushie as LegacyPlushie;
pub use params::Params;

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

    /// As far as I understand, Send and Clone are not compatible
    /// This is a workaround, types implementing the trait can just put Clone::clone() inside
    fn clone(&self) -> Box<dyn PlushieTrait>;
}

pub fn parse_to_any_plushie(
    selector: &str,
    pattern: &str,
) -> Result<Box<dyn PlushieTrait>, String> {
    let inner: Box<dyn PlushieTrait> = match selector {
        "legacy" => Box::new(LegacyPlushie::parse_any_format(pattern)?),
        "flow" => Box::new(Plushie::parse(pattern)?),
        _ => return Err(format!("unrecognized plushie version: {selector}")),
    };
    Ok(inner)
}
