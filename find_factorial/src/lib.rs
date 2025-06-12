pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }

    let mut res: u64 = 1;
    let mut i = 1;
    while i <= num{
        res *= i;
        i += 1;
    }

    res
}
