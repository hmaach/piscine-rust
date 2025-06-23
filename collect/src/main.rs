use collect::*;

fn main() {
    let mut v = [3, 2, 4, 5, 1, 7];
    let mut v_clone = v;

    bubble_sort(&mut v);
    println!("{:?}", v);

    v_clone.sort_unstable();
    println!("{:?}", v_clone);
}
