use convert_case::{Case, Casing};
use edit_distance::edit_distance;

mod edit_distance;

fn is_not_camel_or_snake(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    let camel = s.to_case(Case::Camel).to_ascii_lowercase();
    let snake = s.to_ascii_lowercase().to_case(Case::Snake).to_ascii_lowercase();

    lower != camel && lower != snake
}

pub fn expected_variable(source: &str, target: &str) -> Option<String> {
    dbg!(is_not_camel_or_snake(target));
    if is_not_camel_or_snake(source) || is_not_camel_or_snake(target) {
        return None;
    }

    let src = source.to_ascii_lowercase();
    let tgt = target.to_ascii_lowercase();

    if tgt.len() == 0 || src.len() == 0 {
        return None;
    }

    let dist = edit_distance(&src, &tgt);

    if dist > tgt.len() || dist > src.len() {
        return None;
    }

    let close = ((100.0 * (tgt.len() - dist) as f64) / tgt.len() as f64).round() as u32;

    if close > 50 {
        Some(format!("{}%", close))
    } else {
        None
    }
}
