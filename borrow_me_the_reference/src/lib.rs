pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    let mut to_delete = 0;
    let mut to_backspace = 0;

    while i < s.clone().len() {
        if s.chars().nth(i).unwrap() == '+' {
            to_delete += 1;
            s.replace_range(i..i + 1, "");
        } else if s.chars().nth(i).unwrap() == '-' {
            to_backspace += 1;
            s.replace_range(i..i + 1, "");
        } else if s.chars().nth(i).unwrap() != '+' && to_delete > 0 {
            s.replace_range(i..i + to_delete, "");
            to_delete = 0;
            // i -= to_delete;
        } else if s.chars().nth(i).unwrap() != '-' && to_backspace > 0 {
            s.replace_range(i - to_backspace..i, "");
            to_backspace = 0;
            // i += to_backspace;
        } else {
            i += 1;
        }
    }
    // for (i, ch) in s.clone().chars().enumerate() {
    // }
}

pub fn do_operations(v: &mut [String]) {}
