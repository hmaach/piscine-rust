use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut cals: f64 = 0.;
    let mut carbs: f64 = 0.;
    let mut proteins: f64 = 0.;
    let mut fats: f64 = 0.;

    for food in foods {
        cals += food.calories.1.replace("kcal", "").parse::<f64>().unwrap_or(0.) * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }

    let round = |val: f64| (val * 100.0).round() / 100.0;

    let mut res = json::Null;
    res["cals"] = JsonValue::from(round(cals));
    res["fats"] = JsonValue::from(round(fats));
    res["carbs"] = JsonValue::from(round(carbs));
    res["proteins"] = JsonValue::from(round(proteins));

    return res;
}
