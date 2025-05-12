use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut hash_map: HashMap<&'a str, usize> = HashMap::new();
    for word in words {
        *hash_map.entry(word).or_insert(0) += 1;
    }
    hash_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
