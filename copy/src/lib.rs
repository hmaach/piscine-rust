pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut exps = String::new();

    for nb in a.split_whitespace() {
        exps.push_str(&nb.parse::<f64>().unwrap().exp().to_string());
        exps.push(' ');
    }

    exps.pop();
    (a, exps)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut n_log_abs = Vec::<f64>::with_capacity(b.len());
    for nb in &b {
        n_log_abs.push((*nb as f64).abs().ln())
    }

    (b, n_log_abs)
}
