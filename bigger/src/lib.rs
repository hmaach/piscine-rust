use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values().cloned().into_iter().max().unwrap()
}
