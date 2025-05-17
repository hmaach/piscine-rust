use std::collections::HashMap;

use json;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}
fn round(f: f64) -> f64 {
    dbg!(f);
    dbg!((f * 100.0).round() / 100.0);
    (f * 100.0).round() / 100.0
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut data = HashMap::new();

    for f in foods {
        let calories = &f.calories.1.replace("kcal", "").parse::<f64>().unwrap();
        *data.entry("calories").or_insert(0.) += round(calories * f.nbr_of_portions);
        *data.entry("fats").or_insert(0.) += round(f.fats * f.nbr_of_portions);
        *data.entry("carbs").or_insert(0.) += round(f.carbs * f.nbr_of_portions);
        *data.entry("proteins").or_insert(0.) += round(f.proteins * f.nbr_of_portions);
    }

    *data.entry("calories").or_insert(0.) = round(*data.get("calories").unwrap());


    json::JsonValue::from(data)
}
