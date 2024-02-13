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

impl Pattern {
    #[allow(unused)]
    pub fn tmp_diamond() -> Self {
        use Stitch::Single;
        Self {
            starting_circle: 4,
            ending_circle: 4,
            rounds: vec![vec![Single, Single, Single, Single]],
        }
    }

    #[allow(unused)]
    pub fn tmp_diamond_2() -> Self {
        use Stitch::Single;
        Self {
            starting_circle: 4,
            ending_circle: 4,
            rounds: vec![
                vec![Single, Single, Single, Single],
                vec![Single, Single, Single, Single],
            ],
        }
    }

    #[allow(unused)]
    pub fn tmp_diamond_3() -> Self {
        use Stitch::*;
        Self {
            starting_circle: 4,
            ending_circle: 4,
            rounds: vec![
                vec![Single, Increase, Single, Single],
                vec![Single, Decrease, Single, Single],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let p1 = Pattern::tmp_diamond_3();
        let s = serde_yaml::to_string(&p1).unwrap();
        println!("{s}");
        let p2: Pattern = serde_yaml::from_str(&s).unwrap();
        assert_eq!(p1, p2);
    }
}
