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
        //     // i += to_backspace;
        } else {
            i += 1;
        }
    }
    // for (i, ch) in s.clone().chars().enumerate() {
    // }
}

pub fn do_operations(v: &mut [String]) {
    let mut sum = 0;
    let mut cur_op = '+';
    let mut to_save = String::new();
    for (i, nb_str) in v.iter().enumerate() {
        for ch in nb_str.chars() {
            dbg!(sum);
            if ch == '+' {
                sum += to_save.parse::<i32>().unwrap();
                cur_op = '+';
            } else if ch == '-' {
                sum -= to_save.parse::<i32>().unwrap();
                cur_op = '-';
            } else {
                to_save.push(ch);
            }
        }
        sum = 0;
    }
}
