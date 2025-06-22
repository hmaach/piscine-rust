pub fn num_to_ordinal(x: u32) -> String {
    let mut res = String::from(x.to_string());

    match x {
        0 => res.push_str("th"),
        1 => res.push_str("st"),
        2 => res.push_str("nd"),
        3 => res.push_str("rd"),
        11..=20 => res.push_str("th"),
        n if ((n - 1) % 10 == 0) => res.push_str("st"),
        n if ((n - 2) % 10 == 0) => res.push_str("nd"),
        n if ((n - 3) % 10 == 0) => res.push_str("rd"),
        _ => res.push_str("th"),
    }

    res
}
