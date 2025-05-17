mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let erea = rectangle_area(x, y);
    let shape_erea: f64 = match kind {
        GeometricalShapes::Square => (square_area(a) as f64) * (times as f64),
        GeometricalShapes::Circle => circle_area(a) * (times as f64),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64 * (times as f64),
        GeometricalShapes::Triangle => triangle_area(a, b) * (times as f64),
    };
    erea as f64 >= shape_erea
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let erea = parallelepiped_volume(x, y, z);

    let shape_erea: f64 = match kind {
        GeometricalVolumes::Cube => (cube_volume(a) as f64) * (times as f64),
        GeometricalVolumes::Sphere => sphere_volume(a) * (times as f64),
        GeometricalVolumes::Cone => cone_volume(a, b) as f64 * (times as f64),
        GeometricalVolumes::TriangularPyramid => {
            triangular_pyramid_volume(a as f64, b) * (times as f64)
        }
        GeometricalVolumes::Parallelepiped => {
            parallelepiped_volume(a, b, c) as f64 * (times as f64)
        }
    };

    erea as f64 >= shape_erea
}
