use std::fs;

fn main() {
    let path = "a.txt";

    handling::open_or_create(&path, "content to be written");

    let contents = fs::read_to_string(path).unwrap();
    println!("{}", contents);
}
