/// pointy hexagonal coordinate system.
pub struct HexagonalCoordinate {
    scale: f32,
}

impl HexagonalCoordinate {
    /// Create a new HexagonalCoordinate.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { scale: 1.0 }
    }

    /// Create a new HexagonalCoordinate with a scale.
    #[allow(dead_code)]
    pub fn with_scal(scale: f32) -> Self {
        Self { scale }
    }

    /// Convert a point in the Cartesian coordinate system
    /// to a point in the hexagonal coordinate system.
    #[allow(dead_code)]
    pub fn to_hexagonal(&self, point: (f32, f32)) -> (f32, f32) {
        (
            (3.0_f32.sqrt() * point.0 - point.1) / 3.0 / self.scale,
            2.0 / 3.0 * point.1 / self.scale,
        )
    }

    /// Convert a point in the hexagonal coordinate system
    /// to a point in the Cartesian coordinate system.
    #[allow(dead_code)]
    pub fn to_cartesian(&self, point: (f32, f32)) -> (f32, f32) {
        (
            3.0_f32.sqrt() * (point.0 + 0.5 * point.1) * self.scale,
            1.5 * point.1 * self.scale,
        )
    }

    /// Convert a grid point in the hexagonal coordinate system
    /// to a point in the Cartesian coordinate system.
    #[allow(dead_code)]
    pub fn hexagonal_grid(&self, point: (i32, i32)) -> (f32, f32) {
        (
            3.0_f32.sqrt() * 0.5 * (2 * point.0 + point.1) as f32 * self.scale,
            1.5 * point.1 as f32 * self.scale,
        )
    }

    /// Snap a point in the hexagonal coordinate system
    /// to a grid point in the hexagonal coordinate system.
    #[allow(dead_code)]
    pub fn snap_hexagonal_to_hexagonal_grid(&self, point: (f32, f32)) -> (i32, i32) {
        let (q, r, s) = (point.0, point.1, -point.0 - point.1);
        let [mut q_round, mut r_round, s_round] = [q.round(), r.round(), s.round()];
        let [q_diff, r_diff, s_diff] = [
            (q_round - q).abs(),
            (r_round - r).abs(),
            (s_round - s).abs(),
        ];

        if q_diff > r_diff && q_diff > s_diff {
            q_round = -r_round - s_round;
        } else if r_diff > s_diff {
            r_round = -q_round - s_round;
        }

        (q_round as i32, r_round as i32)
    }

    /// Snap a point in the Cartesian coordinate system
    /// to a grid point in the hexagonal coordinate system.
    #[allow(dead_code)]
    pub fn snap_cartesian_to_hexagonal_grid(&self, point: (f32, f32)) -> (i32, i32) {
        self.snap_hexagonal_to_hexagonal_grid(self.to_hexagonal(point))
    }
}
