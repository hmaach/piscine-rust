use std::io::stdin;

fn main() {
    let mut n_trials = 0;
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";

    loop {
        println!("{}", &riddle);

        let mut answer = String::new();
        match stdin().read_line(&mut answer) {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR: unable to read input: {}", e);
                continue;
            }
        }
        n_trials += 1;
        if answer.trim() == correct_answer {
            println!("Number of trials: {}", n_trials);
            break;
        }
    }
}
