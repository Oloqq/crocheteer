pub enum Stitch {
    Single,
    _Increase,
    _Decrease,
}

pub struct Pattern {
    pub starting_circle: usize,
    pub ending_circle: usize,
    pub rounds: Vec<Vec<Stitch>>,
}

impl Pattern {
    pub fn tmp_diamond() -> Self {
        use Stitch::Single;
        Self {
            starting_circle: 4,
            ending_circle: 4,
            rounds: vec![vec![Single, Single, Single, Single]],
        }
    }
}
