pub fn sum(a: &[i32]) -> i32 {
    a.len().try_into().unwrap()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let arr = [10; 32];
    arr
}
