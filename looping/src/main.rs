fn main() {
    let riddle: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut attempts: u32 = 0;
    let correct_answer: &str = "e";

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
        if s == correct_answer {
            println!("after {} attempts you get it right!", attempts);
            break;
        } else {
            println!("{}", riddle);
        };
        attempts += 1;
    }
}
