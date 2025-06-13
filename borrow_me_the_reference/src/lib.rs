pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    let mut to_delete = 0;
    let mut to_backspace = 0;

    while i < s.len() {
        let cur_ch = s.chars().nth(i).unwrap();

        if cur_ch == '+' {
            to_delete += 1;
            s.replace_range(i..i + 1, "");
        } else if cur_ch == '-' {
            to_backspace += 1;
            s.replace_range(i..i + 1, "");
        } else if cur_ch != '+' && to_delete > 0 {
            s.replace_range(i..i + to_delete, "");
            to_delete = 0;
        } else if cur_ch != '-' && to_backspace > 0 {
            s.replace_range(i - to_backspace..i, "");
            to_backspace = 0;
        } else {
            i += 1;
        }
    }

    if s.contains("+") || s.contains("-") {
        delete_and_backspace(s);
    }
}

pub fn do_operations(v: &mut [String]) {
    for elem in v {
        let mut res = 0;
        if elem.contains("+") {
            let nbs = elem.split_once("+").unwrap();
            res = nbs.0.parse::<i32>().unwrap() + nbs.1.parse::<i32>().unwrap();
        } else if elem.contains("-") {
            let nbs = elem.split_once("-").unwrap();
            res = nbs.0.parse::<i32>().unwrap() - nbs.1.parse::<i32>().unwrap();
        }
        *elem = res.to_string();
    }
}
