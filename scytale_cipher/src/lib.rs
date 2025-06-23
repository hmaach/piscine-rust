pub fn scytale_cipher(message: String, i: u32) -> String {
    let length = message.chars().count();
    if length < 2 || (length as u32) < i {
        return message;
    }

    let mut message_full = String::from(&message);

    while (message_full.chars().count() as u32) % i != 0 {
        message_full.push(' ');
    }

    let mut res = String::new();

    let mut z = 0;
    while z < i {
        let mut j = z as usize;
        while j < message_full.chars().count() {
            match message_full.chars().nth(j as usize) {
                Some(ch) => res.push(ch),
                None => (),
            }

            j += i as usize;
        }
        z += 1;
    }

    res.trim().to_owned()
}
