use areas_volumes::*;

pub mod areas_volumes;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rectangle = rectangle_area(x, y) as f64;

    let area = match kind {
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    if area * times as f64 <= rectangle {
        true
    } else {
        false
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_volume = parallelepiped_volume(x, y, z) as f64;

    let volume = match kind {
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
    };

    if volume * times as f64 <= box_volume {
        true
    } else {
        false
    }
}
