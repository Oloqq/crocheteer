pub type Data = String;

pub trait Simulation {
    fn new() -> Self;
    fn step(&mut self, dt: f32) -> Option<Data>;
    fn react(&mut self, msg: &str);
}
