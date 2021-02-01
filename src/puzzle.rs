/// A ring is a circle of 12 numbers listed in clockwise order
type Ring = [usize; 12];

/// Number of rotations (0 - 11) applied to each layer (base layer doesn't rotate).
/// The topmost layer is index 0, and the bottom layer is index 4.
pub type Rotation = (usize, usize, usize, usize);

/// A 2-D plane of numbers. Since each layer may have a different number of rings,
/// all layers are represented as 4 rings but zeroes are used for non-existent cells.
struct Layer {
    layer: usize,
    rings: [&'static Ring; 4],
}

impl Layer {
    fn new(layer: usize, rings: [&'static Ring; 4]) -> Self {
        Self { layer, rings }
    }

    /// Get the number at a certain location. The column is the position within a ring,
    /// and the row is the ring number with 0 meaning the innermost and 4 meaning the
    /// outermost.
    fn get_cell(&self, column: usize, layer: usize, rotation: &Rotation) -> usize {
        let rotation = match self.layer {
            0 => rotation.0,
            1 => rotation.1,
            2 => rotation.2,
            3 => rotation.3,
            4 => 0,
            _ => unreachable!(),
        };

        let idx = (column + rotation) % 12;
        self.rings[layer][idx]
    }
}

// A bunch of zeroes. Used for rings which aren't physically built
const L0: Ring   = [  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0 ];

// bottom layer, inner ring
const L4_0: Ring = [ 14, 11, 11, 14, 11, 14, 11, 14, 14, 11, 14, 11 ];
const L4_1: Ring = [ 15,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14 ];
const L4_2: Ring = [  9,  4,  4,  6,  6,  3,  3, 14, 14, 21, 21,  9 ];
const L4_3: Ring = [  8,  8,  3,  4, 12,  2,  5, 10,  7, 16,  8,  7 ];

// second layer, inner ring
const L3_0: Ring = [  7, 14, 11,  0,  8,  0, 16,  2,  7,  0,  9,  0 ];
const L3_1: Ring = [  6,  0, 14, 12,  3,  8,  9,  0,  9, 20, 12,  3 ];
const L3_2: Ring = [  2, 13,  9,  0, 17, 19,  3, 12,  3, 26,  6,  0 ];
const L3_3: Ring = [ 12,  0,  6,  0, 10,  0, 10,  0,  1,  0,  9,  0 ];

// third layer, inner ring
const L2_0: Ring = [  7,  8,  9, 13,  9,  7, 13, 21, 17,  4,  5,  0 ];
const L2_1: Ring = [ 12,  0, 21,  6, 15,  4,  9, 18, 11, 26, 14,  1 ];
const L2_2: Ring = [  9,  0,  5,  0, 10,  0,  8,  0, 22,  0, 16,  0 ];

// fourth layer, inner ring
const L1_0: Ring = [  6, 17,  7,  3,  0,  6,  0, 11, 11,  6, 11,  0 ];
const L1_1: Ring = [ 12,  0,  4,  0,  7, 15,  0,  0, 14,  0,  9,  0 ];

// top layer, only layer
const L0_0: Ring = [  8,  0,  3,  0,  6,  0, 10,  0,  7,  0, 15,  0 ];

pub struct Puzzle {
    l0: Layer,
    l1: Layer,
    l2: Layer,
    l3: Layer,
    l4: Layer,
}

impl Puzzle {
    pub fn new() -> Self {
        let l0 = Layer::new(0, [ &L0_0, &L0,   &L0,   &L0   ]);
        let l1 = Layer::new(1, [ &L1_0, &L1_1, &L0,   &L0   ]);
        let l2 = Layer::new(2, [ &L2_0, &L2_1, &L2_2, &L0   ]);
        let l3 = Layer::new(3, [ &L3_0, &L3_1, &L3_2, &L3_3 ]);
        let l4 = Layer::new(4, [ &L4_0, &L4_1, &L4_2, &L4_3 ]);

        Self { l0, l1, l2, l3, l4 }
    }

    pub fn is_solution(&self, rotation: Rotation) -> bool {
        (0..12).into_iter().all(|c| self.is_42(c, &rotation))
    }

    fn is_42(&self, column: usize, rotation: &Rotation) -> bool {
        (0..4).into_iter().map(|r| self.get_cell(column, r, rotation)).sum::<usize>() == 42
    }

    fn get_cell(&self, column: usize, row: usize, rotation: &Rotation) -> usize {
        [ &self.l0, &self.l1, &self.l2, &self.l3, &self.l4 ]
            .iter()
            .find_map(|layer| {
                let res = layer.get_cell(column, row, rotation);
                if res == 0 { None } else { Some(res) }
            })
            .unwrap()
    }

    pub fn print(&self, rotation: &Rotation) {
        for row in 0..4 {
            for column in 0..12 {
                let cell = self.get_cell(column, row, rotation);
                print!("{:2}, ", cell);
            }

            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NEUTRAL: Rotation = (0, 0, 0, 0);

    #[test]
    fn ring_gets_value() {
        let ring = Layer::new(0, [ &L0_0, &L0, &L0, &L0 ]);
        let res = ring.get_cell(0, 0, &NEUTRAL);
        assert_eq!(res, 8);
    }

    #[test]
    fn ring_gets_rotated_value() {
        let ring = Layer::new(0, [ &L0_0, &L0, &L0, &L0 ]);
        let rotation = (2, 0, 0, 0);
        let res = ring.get_cell(0, 0, &rotation);
        assert_eq!(res, 3);
    }
}
