use super::{
    common::{Output, Program},
    execution::Runtime,
    problem::shape::compare_shapes,
};

pub type FitnessFunc = fn(expected: &Output, actual: &Output, runtime: &Runtime) -> f32;

pub fn shape_fitness(expected: &Output, actual: &Output, _runtime: &Runtime) -> f32 {
    -compare_shapes(expected, actual) / actual.point_count() as f32
}

#[cfg(test)]
mod tests {
    use crate::genetic::{evolution::run_and_rank, params::Params};

    use super::*;

    #[test]
    fn test_not_nan() {
        let program = Program::deserialize("(Dec Dec Dec Dec Dec Sc Inc Dec Dec Dec Dec Dec)");
        let params = Params::default();
        // let mut runtime = Runtime::new(&params);
        // let output = runtime.execute(program);
        // let fitness = shape_fitness(targets, &output, &runtime);
    }
}
