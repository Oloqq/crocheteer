pub mod centroid_stuffing;
pub mod link_force;
pub mod simulated_plushie;
pub mod single_loop;

mod initializers;

pub use initializers::Initializer;

pub fn step() {
    // add new nodes for one by one initializer
    // adjust centroid number

    compute_forces();
}

fn compute_forces() {
    // link forces
    // stuffing force
    // single loop force
}
