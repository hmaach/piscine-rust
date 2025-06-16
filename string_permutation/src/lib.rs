use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut hash_map: HashMap<char, i32> = HashMap::new();

    for ch in s1.chars() {
        *hash_map.entry(ch).or_insert(0) += 1;
    }

    for ch in s2.chars() {
        let counter = hash_map.entry(ch).or_insert(0);
        *counter -= 1;
        if *counter < 0 {
            return false;
        }
    }

    true
}
