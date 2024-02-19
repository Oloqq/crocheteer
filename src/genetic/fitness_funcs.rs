use super::{
    common::{Number, Program},
    execution::Runtime,
};
use std::cmp;

pub type FitnessFunc = fn(expected: &Vec<Number>, actual: &Vec<Number>, runtime: &Runtime) -> f32;

pub fn diff_first(expected: &Vec<Number>, actual: &Vec<Number>, _runtime: &Runtime) -> f32 {
    let output = actual.get(0).unwrap_or(&0);
    let expected = expected[0];
    let error = (*output as f32 - expected as f32).abs();
    -error
}

pub fn diff_best(expected: &Vec<Number>, actual: &Vec<Number>, _runtime: &Runtime) -> f32 {
    let mut min_error = f32::MAX;
    for i in 0..cmp::min(expected.len(), actual.len()) {
        let output = actual.get(i).unwrap_or(&0);
        let expected = expected[i];
        let error = (*output as f32 - expected as f32).abs();
        if error < min_error {
            min_error = error;
        }
    }
    -min_error
}

pub fn diff_only(expected: &Vec<Number>, actual: &Vec<Number>, _runtime: &Runtime) -> f32 {
    if actual.len() != 1 {
        return f32::MIN;
    }
    diff_first(expected, actual, _runtime)
}

pub fn diff_first_promote_single(
    expected: &Vec<Number>,
    actual: &Vec<Number>,
    _runtime: &Runtime,
) -> f32 {
    let output = match actual.get(0) {
        Some(x) => x,
        None => return -f32::INFINITY,
    };
    let expected = expected[0];
    let error = (*output as f32 - expected as f32).abs();
    -error * f32::sqrt(actual.len() as f32)
}

#[allow(unused)]
fn distance(a: Number, b: Number) -> f32 {
    a.abs_diff(b) as f32
}

fn distance_square(a: Number, b: Number) -> f32 {
    (a.abs_diff(b) as f32).powi(2)
}

fn promote_output_len(wanted: usize, actual: usize) -> f32 {
    actual.abs_diff(wanted) as f32 + 1.0
}

fn promote_reading(runtime: &Runtime) -> f32 {
    (runtime.input.len().abs_diff(runtime.input_cursor) + 1) as f32
}

fn punish_overreading(runtime: &Runtime) -> f32 {
    if runtime.overread {
        f32::MAX
    } else {
        1.0
    }
}

#[allow(unused)]
pub fn fit_exact_reading(_expected: &Vec<Number>, _actual: &Vec<Number>, runtime: &Runtime) -> f32 {
    -promote_reading(runtime) * punish_overreading(runtime)
}

pub fn diff_first_promote_reading(
    expected: &Vec<Number>,
    actual: &Vec<Number>,
    runtime: &Runtime,
) -> f32 {
    let output = match actual.get(0) {
        Some(x) => x,
        None => return -f32::INFINITY,
    };
    let expected = expected[0];
    let error = distance_square(*output, expected)
        * promote_output_len(1, actual.len())
        * promote_reading(runtime)
        * punish_overreading(runtime);
    -error
}

pub fn fit_arithmetic_series(
    expected: &Vec<Number>,
    actual: &Vec<Number>,
    _runtime: &Runtime,
) -> f32 {
    let mut error: f32 = 0.0;
    for i in 0..cmp::min(expected.len(), actual.len()) {
        let output = match actual.get(i) {
            Some(x) => x,
            None => return f32::MIN,
        };
        error += (*output as f32 - expected[i] as f32).abs();
    }
    error += 1000.0 * (expected.len() as f32 - actual.len() as f32).abs();
    -error
}

pub fn fit_bool(expected: &Vec<Number>, actual: &Vec<Number>, _runtime: &Runtime) -> f32 {
    if actual.len() != 1 {
        return -1.0;
    }
    if expected[0] != 0 && actual[0] != 0 || expected[0] == 0 && actual[0] == 0 {
        return 0.0;
    }
    return -1.0;
}

pub fn normalize_fitness(fitness: &Vec<f32>, _programs: &Vec<Program>) -> Vec<f64> {
    // assert_eq!(fitness.len(), programs.len());
    // let fitness_max = fitness.iter().max_by(|a, b| a.total_cmp(b)).unwrap().clone() as f64;
    // let prog_len_max = programs.iter().map(|p| p.len()).max().unwrap().clone() as f64;
    // fitness.iter().zip(programs).map(|(fit, prog)| {
    //     (1.3 - (prog.len() as f64 / prog_len_max)) * (*fit as f64 / fitness_max)
    // }).collect()
    fitness.iter().map(|f| *f as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_first() {
        let expected = vec![1, 2, 43];
        let inp = vec![1];
        let runtime = Runtime::new(1, &inp, &mut None);
        assert_eq!(
            diff_first(&expected, &vec![1, 0, 12512, 453333], &runtime),
            0.0
        );
        assert_eq!(
            diff_first(&expected, &vec![-1, 0, 12512, 453333], &runtime),
            -2.0
        );
    }

    // #[test]
    // fn test_diff_first() {
    //     let expected = vec![1, 2, 43];
    //     assert_eq!(diff_first(&expected, &vec![1, 0, 12512, 453333]), 0.0);
    //     assert_eq!(diff_first(&expected, &vec![-1, 0, 12512, 453333]), -2.0);
    // }
}
