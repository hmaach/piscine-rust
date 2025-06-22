use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for ch in s.chars() {
        if !chars.contains(&ch) && ch.is_alphabetic() {
            chars.insert(ch);
        }
    }

    chars.len() == 26
}
