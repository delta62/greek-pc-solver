use super::puzzle::Rotation;

pub struct Solver {
    state: Rotation,
}

impl Solver {
    pub fn new() -> Self {
        let state = ( 0, 0, 0, 0 );
        Self { state }
    }
}

impl Iterator for Solver {
    type Item = Rotation;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.state;

        let next = match self.state {
            (w, 11, 11, 11) => (w + 1, 0, 0, 0),
            (w,  x, 11, 11) => (w, x + 1, 0, 0),
            (w,  x,  y, 11) => (w, x, y + 1, 0),
            (w,  x,  y,  z) => (w, x, y, z + 1),
        };

        self.state = next;

        if ret.0 > 11 {
            None
        } else {
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_no_rotations() {
        let mut solver = Solver::new();
        assert_eq!(solver.next(), Some((0, 0, 0, 0)));
    }

    #[test]
    fn generates_last_rotations() {
        let solver = Solver::new();
        assert_eq!(solver.last(), Some((11, 11, 11, 11)));
    }
}
