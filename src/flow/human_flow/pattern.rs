#![allow(unused)]

struct Part {
    // aka generalized cylinder
}

struct Pattern {
    parts: Vec<Part>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bloated_pattern() {
        let src = "
        @centroids = 6
        @floor = true
        R0: MR 6 (6)
        R1: 6 inc (12)
        : [inc, sc] x 6 (18)
        : [inc, 2 sc] x 6 (24)
        : [inc, 3 sc] x 6 (30)
        : [inc, 4 sc] x 6 (36)
        : BLO, 36 sc (36)
        R7-R8: 36 sc (36)
        : [inc, 5 sc] x 6 (42)
        @color = green
        2: 42 sc (42)
        : [inc, 6 sc] x 6 (48)
        ";
    }
}
