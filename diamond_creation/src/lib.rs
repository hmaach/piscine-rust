pub fn get_diamond(c: char) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let size = (c as u8 - b'A') as usize;
    let width = 2 * size + 1;

    for i in 0..=size {
        let current_char = (b'A' + (i as u8)) as char;
        let mut row = vec![' '; width];

        if i == 0 {
            row[size] = current_char;
        } else {
            row[size - i] = current_char;
            row[size + i] = current_char;
        }

        res.push(row.iter().collect());
    }

    for i in (0..size).rev() {
        res.push(res[i].clone());
    }

    res
}
