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
    (f * 100.0).round() / 100.0
}
fn format_float(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if (rounded * 10.0).round() / 10.0 == rounded {
        format!("{:.1}", rounded)
    } else {
        format!("{:.2}", rounded)
    }
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut data: HashMap<&'static str, f64> = HashMap::new();
    let mut data1 = HashMap::new();

    if foods.is_empty() {
        return json::JsonValue::from(data);
    }

    for f in foods {
        let calories = &f.calories.1.replace("kcal", "").parse::<f64>().expect("aaaaaaa");
        *data.entry("calories").or_insert(0.) += calories * f.nbr_of_portions;
        *data.entry("fats").or_insert(0.) += f.fats * f.nbr_of_portions;
        *data.entry("carbs").or_insert(0.) += f.carbs * f.nbr_of_portions;
        *data.entry("proteins").or_insert(0.) += f.proteins * f.nbr_of_portions;
    }

    data1.insert("calories", format_float(*data.get("calories").expect("bbbbbbb")));
    data1.insert("fats", format_float(*data.get("fats").expect("cccccc")));
    data1.insert("carbs", format_float(*data.get("carbs").expect("ddddd")));
    data1.insert("proteins", format_float(*data.get("proteins").expect("eeeeee")));

    json::JsonValue::from(data1)
}

