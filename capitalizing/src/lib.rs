pub fn capitalize_first(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string();
    }
    let mut to_string = input.to_string();
    to_string.replace_range(
        0..1,
        &to_string.chars().nth(0).unwrap().to_string().to_uppercase(),
    );
    to_string
}

pub fn title_case(input: &str) -> String {
    let mut to_string = input.to_string();
    for (i, _) in to_string.clone().chars().enumerate() {
        if i == 0
            || to_string.chars().nth(i - 1) == Some(' ')
            || to_string.chars().nth(i - 1) == Some('\t')
            || to_string.chars().nth(i - 1) == Some('\n')
        {
            to_string.replace_range(
                i..i + 1,
                &to_string.chars().nth(i).unwrap().to_string().to_uppercase(),
            );
        }
    }
    to_string
}
pub fn change_case(input: &str) -> String {
    let mut res = String::new();

    for ch in input.chars() {
        if ch.is_uppercase() {
            res.push_str(&ch.to_lowercase().to_string());
        } else if ch.is_lowercase() {
            res.push_str(&ch.to_uppercase().to_string());
        } else {
            res.push(ch);
        }
    }

    res
}
