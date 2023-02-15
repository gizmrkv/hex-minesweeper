#![allow(dead_code)]

/// Hexagonal table storage.
pub struct HexagonalTable<T> {
    hex_per_side: usize,
    hex_bound: usize,
    buf: Vec<T>,
}

impl<T> HexagonalTable<T>
where
    T: Default + Copy + Clone,
{
    /// Create a HexagonalTable.
    pub fn new(hex_per_side: usize) -> Self {
        let hex_bound = 2 * hex_per_side - 1;
        Self {
            hex_per_side,
            hex_bound,
            buf: vec![Default::default(); hex_bound * hex_bound],
        }
    }

    /// Number of hexagons per side.
    pub fn hexagons_per_side(&self) -> usize {
        self.hex_per_side
    }

    /// Bound of hexagons.
    pub fn hexagons_bound(&self) -> usize {
        self.hex_bound
    }

    /// Get a hexagon reference.
    pub fn get(&self, hex: (i32, i32)) -> Option<&T> {
        if self.is_out_of_bound(hex) {
            None
        } else {
            self.buf
                .get((hex.1 * self.hex_bound as i32 + hex.0) as usize)
        }
    }

    /// Get a mutable hexagon reference.
    pub fn get_mut(&mut self, hex: (i32, i32)) -> Option<&mut T> {
        if self.is_out_of_bound(hex) {
            None
        } else {
            self.buf
                .get_mut((hex.1 * self.hex_bound as i32 + hex.0) as usize)
        }
    }

    /// Check if the given coordinates are outside the table.
    pub fn is_out_of_bound(&self, hex: (i32, i32)) -> bool {
        {
            hex.0 < 0
                || hex.1 < 0
                || hex.0 >= self.hex_bound as i32
                || hex.1 >= self.hex_bound as i32
                || hex.0 + hex.1 < (self.hex_per_side - 1) as i32
                || hex.0 + hex.1 > 3 * (self.hex_per_side - 1) as i32
        }
    }
}
