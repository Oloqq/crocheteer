mod registration;

use super::Part;
use crate::common::*;

type Metric = f32;
type Cost = (Metric, Metric, Metric, Metric);

pub fn sort_by_cost(parts: Vec<Part>) -> (Vec<Part>, Vec<f32>) {
    let costs: Vec<Cost> = parts.iter().map(|p| get_cost(&p)).collect();
    let costs: Vec<f32> = normalize_costs(costs);
    let mut part_cost: Vec<(Part, f32)> = parts.into_iter().zip(costs).collect();

    // low cost first
    part_cost.sort_by(|a, b| a.1.total_cmp(&b.1));

    let (parts, costs): (Vec<Part>, Vec<f32>) = part_cost.into_iter().unzip();
    (parts, costs)
}

fn get_cost(part: &Part) -> Cost {
    let centers: Vec<&V> = part.sections.iter().map(|s| &s.center).collect();
    (
        registration_cost(part),
        fit_cost(part),
        part_length(part),
        turning_angle(&centers),
    )
}

/// Sections are similar to each other
/// Registration is complicated and has multiple pitfalls so gonna see if the algorithm works without this metric
fn registration_cost(_part: &Part) -> Metric {
    0.0
}

/// Mean orient_cost of sections
fn fit_cost(part: &Part) -> Metric {
    part.sections.iter().map(|s| s.orient_cost).sum::<f32>() / part.sections.len() as f32
}

fn part_length(part: &Part) -> Metric {
    match part.sections.len() {
        0 => panic!("no sections in part"),
        1 => 0.0, // this arm is probably redundant
        _ => part
            .sections
            .windows(2)
            .map(|window| window[0].center.metric_distance(&window[1].center))
            .sum(),
    }
}

fn turning_angle(centers: &Vec<&V>) -> Metric {
    match centers.len() {
        0 => panic!("no sections in part"),
        1 | 2 => 0.0, // this arm is probably redundant
        _ => {
            centers
                .windows(3)
                .map(|window| {
                    let d1 = window[1] - window[0];
                    let d2 = window[2] - window[1];
                    d1.angle(&d2)
                })
                .sum::<f32>()
                / (centers.len() - 2) as f32
        }
    }
}

fn normalize(cost: Vec<f32>) -> Vec<f32> {
    let vector = na::DVector::from_vec(cost);
    let mean = vector.mean();
    let std_dev = vector.variance().sqrt();

    if std_dev == 0.0 {
        vector.into_iter().cloned().collect()
    } else {
        vector
            .into_iter()
            .map(|val| (val - mean) / std_dev)
            .collect()
    }
}

fn normalize_costs(costs: Vec<Cost>) -> Vec<f32> {
    let mut result = Vec::with_capacity(costs.len());

    // unzip only works for pairs
    let (regi, fit, length, ang) = costs.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |(mut v1, mut v2, mut v3, mut v4), (a, b, c, d)| {
            v1.push(a);
            v2.push(b);
            v3.push(c);
            v4.push(d);
            (v1, v2, v3, v4)
        },
    );
    let regi = normalize(regi);
    let fit = normalize(fit);
    let length = normalize(length);
    let ang = normalize(ang);

    for i in 0..regi.len() {
        println!("{} + {} - {} + {}", regi[i], fit[i], length[i], ang[i]);
        result.push(regi[i] + fit[i] - length[i] + ang[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turning() {
        let p1 = V::new(0.0, 0.0, 0.0);
        let p2 = V::new(1.0, 1.0, 0.0);
        let p3 = V::new(2.0, 1.0, 0.0);

        let res = turning_angle(&vec![&p1, &p2, &p3]);

        assert_eq!(res.to_degrees(), 45.0);
    }
}
