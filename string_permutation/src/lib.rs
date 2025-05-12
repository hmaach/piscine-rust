use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.to_string().chars().count() != s2.to_string().chars().count() {
        return false;
    }

    let mut s_hash_map1 = HashMap::new();
    let mut s_hash_map2 = HashMap::new();

    for ch in s1.to_string().chars() {
        *s_hash_map1.entry(ch).or_insert(0) += 1;
    }

    for ch in s2.to_string().chars() {
        *s_hash_map2.entry(ch).or_insert(0) += 1;
    }

    for ch in s1.to_string().chars() {
        if s_hash_map1.get(&ch).unwrap() - s_hash_map1.get(&ch).unwrap() != 0{
            return false;
        } 
    }

    true
}
