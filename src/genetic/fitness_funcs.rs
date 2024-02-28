use super::{
    common::{Output, Program},
    execution::Runtime,
    problem::shape::compare_shapes,
};

pub type FitnessFunc = fn(expected: &Output, actual: &Output, runtime: &Runtime) -> f32;

pub fn shape_fitness(expected: &Output, actual: &Output, _runtime: &Runtime) -> f32 {
    -compare_shapes(expected, actual) / actual.point_count() as f32
}
