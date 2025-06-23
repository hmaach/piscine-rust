pub fn pig_latin(text: &str) -> String {
    if text.len() < 2 {
        return text.to_owned();
    }

    let mut res = String::new();
    let begin_vowel = match text.to_owned().chars().nth(0) {
        Some(ch) => is_vowel(ch),
        None => false,
    };
    let mut qu_found = false;

    if !begin_vowel {
        if let Some(index) = text.find("qu") {
            if index == 1 {
                qu_found = true;
            }
        }
        let mut end_str = String::new();
        if !qu_found {
            let mut vowel_found = false;
            for ch in text.to_owned().chars() {
                if !is_vowel(ch) && !vowel_found {
                    end_str.push(ch);
                } else {
                    vowel_found = true;
                    res.push(ch);
                }
            }
        } else {
            res.push_str(&text[3..]);
            match text.to_owned().chars().nth(0) {
                Some(ch) => res.push(ch),
                None => (),
            };
            res.push_str("qu");
        }
        res.push_str(end_str.as_str());
    } else {
        res.push_str(text);
    }
    res.push_str("ay");
    res
}

fn is_vowel(ch: char) -> bool {
    "aAeEiIoOuU".contains(ch)
}
