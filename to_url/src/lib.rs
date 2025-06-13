pub fn to_url(s: &str) -> String {
    let mut res = String::new();
    for ch in s.to_owned().chars() {
        if ch == ' ' {
            res.push_str("%20");
            continue;
        }
        res.push(ch);
    }
    res
}
