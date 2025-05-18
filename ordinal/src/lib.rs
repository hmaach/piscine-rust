pub fn num_to_ordinal(x: u32) -> String {
    let y = x % 10;

    let cardinal = match y {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        _ => "th".to_string(),
    };

    format!("{x}{cardinal}")
}
