pub fn first_subword(mut s: String) -> String {
    let mut start = s.len();
    let stop = s.len();
    for (i, ch) in s.chars().enumerate() {
        if i != 0 && (ch == '_' || ch.is_uppercase()) {
            start = i;
            break;
        }
    }
    s.replace_range(start..stop, "");
    s
}
