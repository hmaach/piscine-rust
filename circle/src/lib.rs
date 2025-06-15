#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self {
            center: Point(x, y),
            radius: r,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn intersect(&self, circle: Circle) -> bool {
        let d = self.center.distance(circle.center);
        if d <= self.radius + circle.radius {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, b: Point) -> f64 {
        ((self.0 - b.0).powi(2) + (self.1 - b.1).powi(2)).sqrt()
    }
}
