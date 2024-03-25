mod from_flow;
mod hook;
mod hook_result;

use crate::flow::Flow;

use super::Plushie;

use from_flow::from_flow;

impl Plushie {
    pub fn from_flow(flow: impl Flow) -> Result<Self, String> {
        from_flow(flow)
    }

    pub fn parse(_pattern: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn _position_based_on(&mut self, _other: &Self) {
        todo!()
    }
}

// fn make_nodes(round_starts: Vec<usize>) -> (Nodes, f32) {
//     // assumption: only one radial axis, how to handle shape of letter Y?
//     let mut prev = 0;
//     let mut y = 0.0;
//     let mut nodes = vec![];

//     // TODO what about the tip
//     for rstart in round_starts {
//         let count = rstart - prev;

//         nodes.append(&mut ring(count, y, 1.0));
//         y += 0.7;

//         prev = rstart;
//     }

//     (nodes, y)
// }

// fn ring(nodes: usize, y: f32, desired_stitch_distance: f32) -> Vec<Point> {
//     let circumference = (nodes + 1) as f32 * desired_stitch_distance;
//     let radius = circumference / (2.0 * PI) / 4.0;

//     let interval = 2.0 * PI / nodes as f32;
//     let mut result: Vec<Point> = vec![];

//     for i in 0..nodes {
//         let rads = interval * i as f32;
//         let x = rads.cos() * radius;
//         let z = rads.sin() * radius;
//         let point = Point::new(x, y, z);
//         result.push(point);
//     }
//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_make_nodes() {
//         let rs = vec![4];
//         let (res, _) = make_nodes(rs);
//         assert_eq!(res.len(), 4);

//         let rs = vec![4, 8];
//         let (res, _) = make_nodes(rs);
//         assert_eq!(res.len(), 8);
//     }
// }
