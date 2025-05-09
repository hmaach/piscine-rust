pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }
    let mut res = 1;
    for i in 0..num {
        res *= i + 1
    }
    res
}
