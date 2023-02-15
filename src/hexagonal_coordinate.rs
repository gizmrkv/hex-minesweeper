#![allow(dead_code)]

/// Convert a point in the Cartesian coordinate system
/// to a point in the hexagonal coordinate system.
pub fn to_hexagonal(point: (f32, f32)) -> (f32, f32) {
    (
        (3.0_f32.sqrt() * point.0 - point.1) / 3.0,
        2.0 / 3.0 * point.1,
    )
}

/// Convert a point in the hexagonal coordinate system
/// to a point in the Cartesian coordinate system.
pub fn to_cartesian(point: (f32, f32)) -> (f32, f32) {
    (3.0_f32.sqrt() * (point.0 + 0.5 * point.1), 1.5 * point.1)
}

/// Convert a grid point in the hexagonal coordinate system
/// to a point in the Cartesian coordinate system.
pub fn hexagonal_grid(point: (i32, i32)) -> (f32, f32) {
    (
        3.0_f32.sqrt() * 0.5 * (2 * point.0 + point.1) as f32,
        1.5 * point.1 as f32,
    )
}

/// Snap a point in the hexagonal coordinate system
/// to a grid point in the hexagonal coordinate system.
pub fn snap_hexagonal_to_hexagonal_grid(point: (f32, f32)) -> (i32, i32) {
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
pub fn snap_cartesian_to_hexagonal_grid(point: (f32, f32)) -> (i32, i32) {
    snap_hexagonal_to_hexagonal_grid(to_hexagonal(point))
}
