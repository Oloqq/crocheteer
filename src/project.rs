use crate::plushie::DisplayMode;

pub struct Project {
    pub pattern: String,
    pub display_mode: DisplayMode,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            pattern: "MR(6)".into(),
            display_mode: Default::default(),
        }
    }
}

impl Project {
    // TODO from yaml file
    pub fn grzib() -> Self {
        Self {
            pattern: indoc::indoc! {"
                @centroids = 3

                MR(6)
                : 6 inc (12)
                3: 12 sc (12)
                mark(cap_start)
                : BLO, 6 dec (6)
                FO

                goto(cap_start), color(255, 255, 0)
                : FLO, 12 inc (24)
                2: 24 sc (24)
                : 12 dec (12)
                : 6 dec (6)
                FO
            "}
            .into(),
            display_mode: DisplayMode::Pattern,
        }
    }
}
