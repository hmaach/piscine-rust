use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let s1_count = s1.to_string().chars().count();
    let s2_count = s2.to_string().chars().count();
    if s1_count != s2_count {
        return false;
    }

    if s1_count == 1 && s1 != s2 {
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
        let s2_char = s_hash_map2.get(&ch);
        if !s2_char.is_none() {
            if s_hash_map1.get(&ch).unwrap() - s2_char.unwrap() != 0 {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
