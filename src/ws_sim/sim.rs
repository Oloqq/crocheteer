use std::sync::{Arc, Mutex};

pub type Data = String;

pub trait Simulation: Send + 'static {
    fn step(&mut self, dt: f32) -> Option<Data>;
    fn react(&mut self, msg: &str);
    fn messages(&self) -> Arc<Mutex<Vec<String>>>;
    fn clone(&self) -> Self;
}
