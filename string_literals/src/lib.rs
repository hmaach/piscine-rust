pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for char in v.chars() {
        if !char.is_ascii() {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat) {
        return true;
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("not found")
}
