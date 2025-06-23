use panic::*;
use std::fs::{self, File};

fn main() {
    let filename = "created.txt";
    File::create(filename).unwrap();

    println!("{:?}", open_file(filename));

    fs::remove_file(filename).unwrap();

    // this should panic!
    open_file(filename);
}