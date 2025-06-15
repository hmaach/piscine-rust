use std::collections::HashMap;

pub fn arrange_phrase(phrase: &str) -> String {
    let words = phrase.split_whitespace();
    let mut res = String::new();
    let mut words_map: HashMap<u32, String> = HashMap::new();

    for word in words {
        let mut nb = String::new();
        let mut string = String::new();

        for ch in word.chars() {
            if ch.is_numeric() {
                nb.push(ch);
            } else {
                string.push(ch);
            }
        }
        words_map.insert(nb.parse::<u32>().unwrap(), string);
    }

    let mut entries: Vec<_> = words_map.iter().collect();
    entries.sort_by_key(|&(k, _)| k);

    for (_, value) in entries {
        res.push_str(value);
        res.push(' ');
    }

    res.pop();
    res
}
