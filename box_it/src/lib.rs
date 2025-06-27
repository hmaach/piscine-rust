pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut parts: Box<Vec<u32>> = Box::new(Vec::new());
    for nb in s.split_ascii_whitespace() {
        if nb.ends_with("k") {
            let nbr: f32 = nb
                .to_owned()
                .strip_suffix("k")
                .unwrap()
                .parse::<f32>()
                .unwrap_or(0.)
                * 1000.;
            parts.push(nbr.round() as u32);
        } else {
            parts.push(nb.parse::<u32>().unwrap_or(0));
        }
    }
    parts
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
