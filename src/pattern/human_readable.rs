use super::{stitches::count_anchors_produced, Pattern, Stitch};

type ParseError = String;

impl Pattern {
    #[allow(unused)]
    pub fn human_readable(&self) -> String {
        let mut result = String::with_capacity(100);
        result += format!(
            "R1: MR {} ({})\n",
            self.starting_circle, self.starting_circle
        )
        .as_str();

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

    #[allow(unused)]
    pub fn from_human_readable(text: &str) -> Result<Self, String> {
        let lines: Vec<&str> = text.split("\n").collect();
        let mut lines = lines.iter().enumerate();
        let (mut lnum, mut line) = lines.next().expect("content shouldn't be empty");
        while line.trim().starts_with("#") {
            (lnum, line) = lines.next().expect("EOF");
        }
        let starting_circle = parse_starter(line).unwrap();

        let mut rounds: Vec<Vec<Stitch>> = vec![];
        for (lnum, line) in lines {
            if line.trim() == "FO" {
                break;
            }

            let (repetitions, stitches) = parse_line(line)?;
            for i in 0..repetitions {
                rounds.push(stitches.clone());
            }
        }

        Ok(Self {
            starting_circle,
            ending_circle: count_anchors_produced(rounds.last().unwrap()),
            rounds,
        })
    }
}

fn parse_starter(line: &str) -> Result<usize, ParseError> {
    let tokens: Vec<&str> = line.split(" ").collect();
    return if tokens.len() != 4 {
        Err("Syntax error: expected starting round".into())
    } else if tokens[1].to_ascii_uppercase() != "MR" {
        Err("Expected a magic ring at the start".into())
    } else {
        if let Ok(num) = tokens[2].parse() {
            Ok(num)
        } else {
            Err("Couldn't parse a number".into())
        }
    };
}

fn get_repetitions(roundspec: &str) -> usize {
    match roundspec.split_once("-") {
        None => 1,
        Some((lhs, rhs)) => {
            let lhs_num: usize = lhs.trim()[1..].parse().unwrap();
            let rhs_num: usize = rhs.trim()[1..].parse().unwrap();
            rhs_num - lhs_num + 1
        }
    }
}

fn parse_stitches(stitches_str: &str) -> Result<Vec<Stitch>, ParseError> {
    let tokens = stitches_str.split(", ");
    let mut result = vec![];
    for token in tokens {
        let reps: usize;
        let stitch_str: &str;
        match token.trim().split_once(" ") {
            Some((num_str, stitch_str_1)) => {
                reps = num_str.trim().parse().unwrap();
                stitch_str = stitch_str_1
            }
            None => {
                reps = 1;
                stitch_str = token;
            }
        }

        let stitch = Stitch::from_str(stitch_str).expect("not recognized stitch");
        for _ in 0..reps {
            result.push(stitch);
        }
    }

    Ok(result)
}

fn parse_line(line: &str) -> Result<(usize, Vec<Stitch>), ParseError> {
    let (roundspec, rest) = line.split_once(":").unwrap();
    let repetitions = get_repetitions(roundspec);
    let (stitches, anchors_str) = rest.split_once("(").unwrap();
    let anchors: usize = anchors_str
        .trim()
        .strip_suffix(")")
        .unwrap()
        .parse()
        .unwrap();
    let stitches = parse_stitches(stitches)?;
    assert!(anchors == count_anchors_produced(&stitches));

    Ok((repetitions, stitches))
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
        format!("R{}-R{}", rep + 1, this_round + 1)
    } else {
        format!("R{}", this_round + 1)
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
            ending_circle: 7,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };

        let expected = "R1: MR 6 (6)
R2: 5 sc, inc (7)
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

        let expected = "R1: MR 6 (6)
R2: 3 sc, inc, dec (6)
R3-R5: 6 sc (6)
R6: 3 sc, inc, dec (6)
FO
";
        assert_eq!(p.human_readable().as_str(), expected);
    }

    #[test]
    fn test_get_repetitions() {
        assert_eq!(get_repetitions("R2"), 1);
        assert_eq!(get_repetitions("R2-R4"), 3);
    }

    #[test]
    fn test_loading_basic() {
        let src = "R1: MR 6 (6)
        R2: 5 sc, inc (7)
        FO
        ";

        let expected = Pattern {
            starting_circle: 6,
            ending_circle: 7,
            rounds: vec![vec![Sc, Sc, Sc, Sc, Sc, Inc]],
        };
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }

    #[test]
    fn test_loading_repeated() {
        let src = "R1: MR 6 (6)
        R2: 3 sc, inc, dec (6)
        R3-R5: 6 sc (6)
        R6: 3 sc, inc, dec (6)
        FO
        ";

        let expected = Pattern {
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
        assert_eq!(Pattern::from_human_readable(src).unwrap(), expected);
    }
}
