pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    for ch in v.chars() {
        if !ch.is_ascii() {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}
