use border_cross::*;

fn main() {
    let vehicles: Vec<&dyn Vehicle> = vec![
        &Car {
            plate_nbr: "A3D5C7",
            model: "Model 3",
            horse_power: 325,
            year: 2010,
        },
        &Truck {
            plate_nbr: "V3D5CT",
            model: "Ranger",
            horse_power: 325,
            year: 2010,
            load_tons: 40,
        },
    ];
    println!("{:?}", all_models(vehicles));
}
