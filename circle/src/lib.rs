#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
use std::f64::consts::PI;

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: radius,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        (self.radius).powi(2) * PI
    }
    pub fn intersect(&self, other: Circle) -> bool {
        ((self.center.0 - other.center.0) * (self.center.0 - other.center.0)
            + (self.center.1 - other.center.1) * (self.center.1 - other.center.1))
            .sqrt()
            > 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}
