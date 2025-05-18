pub fn rotate(input: &str, key: i8) -> String {
    let mut step = key;
    if key > 26 {
        step = key % 26
    }

    dbg!((('S' as u8) + (5 % 26) as u8) as char);

    input
        .to_string()
        .chars()
        .map(|c| match c {
            'a'..='m' => ((c as u8) + (key % 26) as u8) as char,
            'n'..='z' => ((c as u8) - (key % 26) as u8) as char,
            'A'..='M' => ((c as u8) + (key % 26) as u8) as char,
            'N'..='Z' => ((c as u8) - (key % 26) as u8) as char,
            _ => c,
        })
        .collect()
}
