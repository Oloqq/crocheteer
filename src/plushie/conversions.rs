use super::Plushie;
use crate::common::*;

impl Plushie {
    pub fn to_mesh(&self) -> Mesh {
        let mut result: Mesh = vec![];

        for (i, neibs) in self.edges.iter().enumerate() {
            if neibs.len() < 2 {
                break;
            }
            for j in 0..neibs.len() - 1 {
                result.push(make_triangle(
                    self.points[i],
                    self.points[neibs[j]],
                    self.points[neibs[j + 1]],
                ))
            }
            if neibs.len() > 2 {
                result.push(make_triangle(
                    self.points[i],
                    self.points[neibs[0]],
                    self.points[neibs[neibs.len() - 1]],
                ))
            }
        }

        result
    }
}
