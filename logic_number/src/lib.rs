pub fn number_logic(num: u32) -> bool {
    let num_str = String::from(num.to_string());
    let mut res = 0;

    for ch in num_str.chars() {
        res += (ch.to_string().parse::<u32>().unwrap_or(0)).pow(num_str.len() as u32);
    }

    res == num
}
