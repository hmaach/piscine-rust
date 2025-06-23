pub fn spell(n: u64) -> String {
    return match n {
        0..100 => spell_tens(n),
        100..1000 => spells_hundreds(n),
        1000..1000000 => spell_thousands(n),
        _ => "one million".to_string(),
    };
}

fn spell_tens(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "fifteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineeen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => {
            let rem = n % 10;
            format!("{}-{}", spell(n - rem), spell(rem))
        }
    }
}

fn spells_hundreds(n: u64) -> String {
    let hundreds = n / 100;
    let rem = n % 100;

    if rem == 0 {
        format!("{} hundred", spell_tens(hundreds))
    } else {
        format!("{} hundred {}", spell_tens(hundreds), spell_tens(rem))
    }
}

fn spell_thousands(n: u64) -> String {
    let thousands = n / 1000;
    let rem = n % 1000;

    if rem == 0 {
        format!("{} thousand", spell(thousands))
    } else {
        format!("{} thousand {}", spell(thousands), spell(rem))
    }
}
