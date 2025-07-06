pub fn add_curry(nb: i32) -> impl Fn(i32) -> i32 {
    move |b| nb + b
}

pub fn twice<F>(f: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    move |x: i32| {
        let apply_twice = |input| f(f(input));
        apply_twice(x)
    }
}
