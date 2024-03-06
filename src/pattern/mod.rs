pub mod builder;
pub mod genetic;
pub mod human_readable;
pub mod protopattern;
pub mod stitches;

use serde::{Deserialize, Serialize};

pub use self::stitches::Stitch;

// enum Starter {
//     MagicRing(usize),
//     Chain(usize),
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Pattern {
    pub starting_circle: usize,
    pub fasten_off: bool,
    pub ending_circle: usize,
    pub rounds: Vec<Vec<Stitch>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        use Stitch::*;
        let p1 = Pattern {
            starting_circle: 4,
            fasten_off: true,
            ending_circle: 4,
            rounds: vec![vec![Sc, Inc, Sc, Sc], vec![Sc, Dec, Sc, Sc]],
        };
        let s = serde_yaml::to_string(&p1).unwrap();
        println!("{s}");
        let p2: Pattern = serde_yaml::from_str(&s).unwrap();
        assert_eq!(p1, p2);
    }
}
