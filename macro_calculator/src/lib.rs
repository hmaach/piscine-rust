pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut calories_total = 0.0;
    let mut fats_total = 0.0;
    let mut carbs_total = 0.0;
    let mut proteins_total = 0.0;

    for food in foods {
        let cal = food
            .calories
            .1
            .trim_end_matches("kcal")
            .parse::<f64>()
            .unwrap_or(0.);
        calories_total += cal * food.nbr_of_portions;
        fats_total += food.fats * food.nbr_of_portions;
        carbs_total += food.carbs * food.nbr_of_portions;
        proteins_total += food.proteins * food.nbr_of_portions;
    }

    calories_total = (calories_total * 100.0).round() / 100.0;
    fats_total = (fats_total * 100.0).round() / 100.0;
    carbs_total = (carbs_total * 100.0).round() / 100.0;
    proteins_total = (proteins_total * 100.0).round() / 100.0;

    let mut res = json::Null;
    res["cals"] = calories_total.into();
    res["fats"] = fats_total.into();
    res["carbs"] = carbs_total.into();
    res["proteins"] = proteins_total.into();

    return res;
}