pub fn pig_latin(text: &str) -> String {
    if text.len() < 2 {
        return text.to_string();
    }

    let mut res = String::new();
    let begin_vowel = is_vowel(text.to_string().chars().nth(0).unwrap());
    let mut qu_found = false;

    if !begin_vowel {
        if let Some(index) = text.find("qu") {
            if index == 1 {
                qu_found = true;
            }
        }
        let mut to_insert_in_the_end = String::new();
        if !qu_found {
            let mut vowel_found = false;
            for ch in text.to_string().chars() {
                if !is_vowel(ch) && !vowel_found {
                    to_insert_in_the_end.push(ch);
                } else {
                    vowel_found = true;
                    res.push(ch);
                }
            }
        } else {
            res.push_str(&text[3..]);
            res.push(text.to_string().chars().nth(0).unwrap());
            res.push_str("qu");
        }
        res.push_str(to_insert_in_the_end.as_str());
    } else {
        res.push_str(text);
    }
    res.push_str("ay");
    res
}

fn is_vowel(ch: char) -> bool {
    "aAeEiIoOuU".contains(ch)
}
