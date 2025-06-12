pub fn rev_str(input: &str) -> String {
    let res: String = input.to_string().chars().rev().collect();
    res
}
