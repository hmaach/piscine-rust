pub fn first_fifty_even_square() -> Vec<i32> {
    (1..)
        .filter(|x: &i32| x % 2 == 0)
        .map(|x: i32| x.pow(2))
        .take(50)
        .collect()
}
