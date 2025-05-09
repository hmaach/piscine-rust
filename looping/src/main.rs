fn main() {
    let riddle: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut attempts: u32 = 0;
    let correct_answer: &str = "The letter e";

    println!("{}", riddle);
    loop {
        use std::io::{Write, stdin, stdout};
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        attempts += 1;
        if s == correct_answer {
            println!("Number of trials: {}", attempts);
            break;
        } else {
            println!("{}", riddle);
        };
    }
}
