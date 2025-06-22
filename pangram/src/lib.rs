use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for ch in s.chars() {
        let lower_ch = ch.to_ascii_lowercase();
        if !chars.contains(&lower_ch) && lower_ch >= 'a' && lower_ch <= 'z' {
            chars.insert(lower_ch);
        }
    }

    chars.len() == 26
}
