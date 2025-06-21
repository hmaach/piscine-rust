pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    TriangularPyramid,
    Parallelepiped,
}

pub(crate) fn square_area(side: usize) -> usize {
    side.pow(2)
}

pub(crate) fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

pub(crate) fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}

pub(crate) fn rectangle_area(side_a: usize, side_b: usize) -> usize {
    side_a * side_b
}

pub(crate) fn cube_volume(side: usize) -> usize {
    side.pow(3)
}

pub(crate) fn sphere_volume(radius: usize) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

pub(crate) fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
    (base_area * height as f64) / 3.0
}

pub(crate) fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
    side_a * side_b * side_c
}

pub(crate) fn cone_volume(base_radius: usize, height: usize) -> f64 {
    (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}
