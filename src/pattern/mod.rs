mod construction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Stitch {
    Single,
    Increase,
    Decrease,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Pattern {
    pub starting_circle: usize,
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
            ending_circle: 4,
            rounds: vec![
                vec![Single, Increase, Single, Single],
                vec![Single, Decrease, Single, Single],
            ],
        };
        let s = serde_yaml::to_string(&p1).unwrap();
        println!("{s}");
        let p2: Pattern = serde_yaml::from_str(&s).unwrap();
        assert_eq!(p1, p2);
    }
}
