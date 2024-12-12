use crate::common::colors;

pub type Label = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Sc,
    Inc,
    Dec,
    Ch(usize),
    /// Create a chain, then attach it to a marked position
    Attach(Label, usize),
    /// Begin working in the other direction
    Reverse,
    /// Front loop only
    FLO,
    /// Back loop only
    BLO,
    /// Both loops
    BL,
    /// Let go of the yarn, start working elsewhere
    Goto(Label),
    /// Mark a spot that will be important later
    Mark(Label),
    /// Magic ring
    MR(usize),
    /// Fasten off
    FO,
    /// Change yarn color
    Color(colors::Color),
}

impl Action {
    pub fn parse(src: &str) -> Option<Self> {
        use Action::*;
        // println!("{src}");
        let mut tokens = src.split(" ");
        let first = tokens.next().unwrap();

        Some(match first {
            "sc" => Sc,
            "inc" => Inc,
            "dec" => Dec,
            // "ch(usize)," => Ch,
            // "attach(Label)," => Attach,
            // "reverse," => Reverse,
            // "flo," => FLO,
            // "blo," => BLO,
            // "bl," => BL,
            // "goto(Label)," => Goto,
            // "mark(Label)," => Mark,
            "MR" => {
                let num: usize = match tokens.next() {
                    Some(x) => match x.parse() {
                        Ok(x) => x,
                        Err(_) => return None,
                    },
                    None => return None,
                };
                MR(num)
            }
            // "fo," => FO,
            // "color(Color)," => Color,
            _ => return None,
        })
    }
}
