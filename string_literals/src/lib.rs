pub fn is_empty(v: &str) -> bool {
    v.to_owned().is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    for ch in v.to_owned().chars() {
        if !ch.is_ascii() {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.to_owned().contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    v.to_owned().find(pat).unwrap()
}

