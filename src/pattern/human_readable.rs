use super::{stitches::count_anchors_produced, Pattern, Stitch};

impl Pattern {
    #[allow(unused)]
    pub fn human_readable(&self) -> String {
        let mut result = String::with_capacity(100);
        result += format!("MR {}\n", self.starting_circle).as_str();

        let mut repetition_start: Option<usize> = None;
        let mut last_round = self.rounds.first().unwrap();
        for (i, round) in self.rounds.iter().enumerate().skip(1) {
            if *round == *last_round {
                if repetition_start.is_none() {
                    repetition_start = Some(i);
                }
            } else {
                result += format!(
                    "{}: {} ({})\n",
                    serialize_round_id(i, repetition_start),
                    serialize_stitches(last_round),
                    count_anchors_produced(last_round)
                )
                .as_str();
                repetition_start = None;
            }
            last_round = round;
        }
        result += format!(
            "{}: {} ({})\n",
            serialize_round_id(self.rounds.len(), repetition_start),
            serialize_stitches(last_round),
            count_anchors_produced(last_round)
        )
        .as_str();

        result += format!("FO\n").as_str();
        result
    }

    pub fn from_human_readable(text: &String) -> Self {
        // let starting = 6;
        todo!()
    }
}

fn serialize_stitches(stitches: &Vec<Stitch>) -> String {
    let mut result: String = String::with_capacity(stitches.len() * 4);
    let mut reps: usize = 1;
    let mut last_stitch = stitches.first().unwrap();
    for stitch in stitches.iter().skip(1) {
        if stitch == last_stitch {
            reps += 1;
        } else {
            result += format!(
                "{} {},",
                if reps > 1 {
                    reps.to_string()
                } else {
                    String::new()
                },
                last_stitch
            )
            .as_str();
            last_stitch = stitch;
            reps = 1;
        }
    }
    result += format!(
        "{} {}",
        if reps > 1 {
            reps.to_string()
        } else {
            String::new()
        },
        last_stitch
    )
    .as_str();

    result
}

fn serialize_round_id(this_round: usize, repetition_start: Option<usize>) -> String {
    if let Some(rep) = repetition_start {
        format!("R{}-R{}", rep, this_round)
    } else {
        format!("R{}", this_round)
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::Stitch;
    use pretty_assertions::assert_eq;
    use Stitch::*;

    use super::*;

    #[test]
    fn test_serialization_basic() {
        let p = Pattern {
            starting_circle: 6,
            ending_circle: 6,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };

        let expected = "MR 6
R1: 5 sc, inc (7)
FO
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    #[test]
    fn test_serialization_repeated() {
        let p = Pattern {
            starting_circle: 6,
            ending_circle: 6,
            rounds: vec![
                vec![Sc, Sc, Sc, Inc, Dec],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Sc, Sc, Sc],
                vec![Sc, Sc, Sc, Inc, Dec],
            ],
        };

        let expected = "MR 6
R1: 3 sc, inc, dec (6)
R2-R4: 6 sc (6)
R5: 3 sc, inc, dec (6)
FO
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    // #[test]
    // fn test_loading_from_human_readable_basic() {
    //     let expected = "MR 6
    //     R1: 6 sc (6)
    //     FO
    //     ";
    // }

    // #[test]
    // fn test_loading_from_human_readable_repeated() {
    //     let expected = "MR 6
    //     R1: 6 sc (6)
    //     FO
    //     ";
    // }
}
