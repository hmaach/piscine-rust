use banner::*;
use std::collections::HashMap;

fn main() {
    let mut handler = FlagsHandler { flags: HashMap::new() };

    let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
    let r = Flag::opt_flag(
        "remainder",
        "remainder of the division between two values, formula (a % b)",
    );

    handler.add_flag(d, div);
    handler.add_flag(r, rem);

    println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

    println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

    println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

    println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
}