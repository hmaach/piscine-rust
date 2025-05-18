pub fn stars(n: u32) -> String {
    let mut res = String::new();

    let mut i = 0;
    while i < 2_i32.pow(n) {
        res.push('*');
        i += 1;
    }

    res
}
