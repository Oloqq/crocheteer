pub type Data = String;

pub trait Simulation: Clone + Send + 'static {
    fn step(&mut self, dt: f32) -> Option<Data>;
    fn react(&mut self, msg: &str);
}
