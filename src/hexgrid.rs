use bevy::prelude::Vec2;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointyHexGrid {
    pub x: i32,
    pub y: i32,
}

impl PointyHexGrid {
    pub fn pointy_hex_grid_to_cartesian(&self) -> Vec2 {
        Vec2 {
            x: 3.0_f32.sqrt() * 0.5 * (2 * self.x + self.y) as f32,
            y: 1.5 * self.y as f32,
        }
    }
}

pub fn cartesian_to_pointy_hex(cartesian_point: Vec2) -> Vec2 {
    Vec2 {
        x: (3.0_f32.sqrt() * cartesian_point.x - cartesian_point.y) / 3.0,
        y: 2.0 / 3.0 * cartesian_point.y,
    }
}

pub fn pointy_hex_to_cartesian(pointy_hex_point: Vec2) -> Vec2 {
    Vec2 {
        x: 3.0_f32.sqrt() * (pointy_hex_point.x + 0.5 * pointy_hex_point.y),
        y: 1.5 * pointy_hex_point.y,
    }
}

pub fn cartesian_point_to_nearest_pointy_hex_grid(cartesian_point: Vec2) -> PointyHexGrid {
    pointy_hex_point_to_nearest_pointy_hex_grid(cartesian_to_pointy_hex(cartesian_point))
}

pub fn pointy_hex_grid_to_cartesian(grid: PointyHexGrid) -> Vec2 {
    Vec2 {
        x: 3.0_f32.sqrt() * 0.5 * (2 * grid.x + grid.y) as f32,
        y: 1.5 * grid.y as f32,
    }
}

pub fn pointy_hex_point_to_nearest_pointy_hex_grid(pointy_hex_point: Vec2) -> PointyHexGrid {
    let (q, r, s) = (
        pointy_hex_point.x,
        pointy_hex_point.y,
        -pointy_hex_point.x - pointy_hex_point.y,
    );
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

    PointyHexGrid {
        x: q_round as i32,
        y: r_round as i32,
    }
}
