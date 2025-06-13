pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(names.len());

    for name in names {
        let mut initial = String::new();

        for part in name.split_whitespace() {
            initial.push(part.chars().nth(0).unwrap());
            initial.push_str(". ");
        }

        initial.pop();
        res.push(initial);
    }
    res
}
