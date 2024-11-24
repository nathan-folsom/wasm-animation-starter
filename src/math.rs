use crate::constants::{ORIGIN, SPIN_V};
use std::f64::consts::PI;

pub struct Rotor {
    pub r: f64,
    pub theta: f64,
    pub v: f64,
    pub l: f64,
    pub c_r: f64,
    pub c_theta: f64,
    pub c_v: f64,
    pub origin_offset: (f64, f64),
}

impl Rotor {
    pub fn get_point(&self) -> (f64, f64) {
        let (cx, cy) = self.get_center();
        Self::get_rotational_point(self.theta, self.r, cx, cy)
    }

    pub fn get_center(&self) -> (f64, f64) {
        Self::get_rotational_point(
            self.c_theta,
            self.c_r,
            ORIGIN.0 + self.origin_offset.0,
            ORIGIN.1 + self.origin_offset.1,
        )
    }

    fn get_rotational_point(theta: f64, r: f64, cx: f64, cy: f64) -> (f64, f64) {
        // sin(theta) = dy / r
        let dy = theta.sin() * r;
        // y = cy + dy
        let y = cy + dy;

        // cos(theta) = dx / r
        let dx = theta.cos() * r;
        // x = cx + dx
        let x = cx + dx;

        (x, y)
    }

    pub fn advance(&mut self) {
        // Spin point around the rotor
        self.theta += self.v;

        // Translate rotor around center of canvas
        self.c_theta += self.c_v;
    }
}

pub fn get_intersection(a: &Rotor, b: &Rotor) -> ((f64, f64), (f64, f64)) {
    let r1 = a.l;
    let r2 = b.l;

    let (x1, y1) = a.get_point();

    let (x2, y2) = b.get_point();

    let cdx = x1 - x2;
    let cdy = y1 - y2;

    let dist = (cdx * cdx + cdy * cdy).sqrt();

    let dist2 = dist * dist;
    let dist4 = dist2 * dist2;

    let a = (r1 * r1 - r2 * r2) / (2.0 * dist2);
    let r1r2 = r1 * r1 - r2 * r2;
    let c = (2.0 * (r1 * r1 + r2 * r2) / dist2 - (r1r2 * r1r2) / dist4 - 1.0).sqrt();

    let fx = (x1 + x2) / 2.0 + a * (x2 - x1);
    let gx = c * (y2 - y1) / 2.0;

    let fy = (y1 + y2) / 2.0 + a * (y2 - y1);
    let gy = c * (x1 - x2) / 2.0;

    ((fx + gx, fy + gy), (fx - gx, fy - gy))
}
