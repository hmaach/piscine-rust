use arrays::*;

fn main() {
    let a = [1,2,3];
    let b = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5];

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}