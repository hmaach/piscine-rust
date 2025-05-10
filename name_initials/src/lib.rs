pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut names_initials: Vec<String> = Vec::new();

    for name in names {
        let mut new_name: String = String::new();
        for (i, elem) in name.to_string().split_whitespace().enumerate() {
            let ch = elem.chars().nth(0).unwrap();
            new_name.push(ch);
            new_name.push('.');
            if i == 0 {
                new_name.push(' ');
            }
        }
        names_initials.push(new_name);
    }
    names_initials
}
