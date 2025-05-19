pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();

    for ch in input.to_string().chars() {
        let new_ch = match ch {
            'a'..='z' => {
                let mut step = 0;
                if key >= 0 {
                    if (((ch as u8) + ((key % 26) as u8)) as char) > 'z' {
                        step = (ch as u8) - (('z' as u8) - ('a' as u8) + 1) + ((key % 26) as u8);
                    } else {
                        step = (ch as u8) + ((key % 26) as u8);
                    }
                } else {
                    if (((ch as u8) - ((key.abs() % 26) as u8)) as char) < 'a' {
                        step =
                            (ch as u8) + (('z' as u8) - ('a' as u8) + 1) - ((key.abs() % 26) as u8);
                    } else {
                        step = (ch as u8) - ((key.abs() % 26) as u8);
                    }
                }
                step as char
            }
            'A'..='Z' => {
                let mut step = 0;
                if key >= 0 {
                    if (((ch as u8) + ((key % 26) as u8)) as char) > 'Z' {
                        step = (ch as u8) - (('Z' as u8) - ('A' as u8) + 1) + ((key % 26) as u8);
                    } else {
                        step = (ch as u8) + ((key % 26) as u8);
                    }
                } else {
                    if (((ch as u8) - ((key.abs() % 26) as u8)) as char) < 'A' {
                        step =
                            (ch as u8) + (('Z' as u8) - ('A' as u8) + 1) - ((key.abs() % 26) as u8);
                    } else {
                        step = (ch as u8) - ((key.abs() % 26) as u8);
                    }
                }
                step as char
            }
            _ => ch,
        };

        res.push(new_ch);
    }
    res
}
