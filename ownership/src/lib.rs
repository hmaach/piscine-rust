pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if (ch.is_uppercase() && i != 0) || ch == '_' {
            break;
        }
        result.push(ch)
    }
    result
}
