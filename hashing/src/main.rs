use hashing::*;

fn main() {
    let v = [4, 7, 5, 2, 5, 1, 3];

    println!("mean {}", mean(&v));
    println!("median {}", median(&v));
    println!("mode {}", mode(&v));
}