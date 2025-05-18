pub fn number_logic(num: u32) -> bool {
    let string_num = num.to_string();
    let length = string_num.len();
    let mut sum = 0;

    for n in string_num.chars() {
        sum += (n as u32 - '0' as u32).pow(length as u32);
    }

    if sum != num {
        return false;
    }

    true
}
