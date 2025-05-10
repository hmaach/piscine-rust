pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let e: f64 = c.abs().pow(c.abs().try_into().unwrap()).into();
    let natural_logarithm_absolute_v: f64 = (c as f64).abs().ln();

    (c, e, natural_logarithm_absolute_v)
}

pub fn str_function(a: String) -> (String, String) {
    let mut exps: String = String::new();
    let nbs: Vec<String> = a.split_whitespace().map(str::to_string).collect();
    for (i, str_nb) in nbs.iter().enumerate() {
        let nb = str_nb.parse::<f64>().unwrap().abs().exp();
        exps.push_str(nb.to_string().as_str());
        if i != nbs.len() - 1 {
            exps.push(' ');
        }
    }
    (a, exps)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result: Vec<f64> = Vec::new();

    for elem in b.clone() {
        result.push((elem as f64).abs().ln());
    }

    (b, result)
}
