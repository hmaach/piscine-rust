use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    if s.len() < 26 {
        return false;
    }
    let mut hash_map: HashMap<char, u32> = HashMap::new();
    for ch in s.to_string().chars() {
        if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') {
            *hash_map.entry(ch).or_insert(0) += 1;
        }
    }

    if hash_map.len() < 26 {
        return false;
    }

    true
}
