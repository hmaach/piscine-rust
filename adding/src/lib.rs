pub fn add_curry(nb: i32) -> impl Fn(i32) -> i32 {
    move |b| nb + b
}
