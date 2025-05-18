pub fn score(s: &str) -> u64 {
    let mut score = 0;

    for ch in s.to_string().chars() {
        match ch {
            'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u' | 'L' | 'l' | 'N' | 'n'
            | 'R' | 'r' | 'S' | 's' | 'T' | 't' => score += 1,
            'D' | 'd' | 'G' | 'g' => score += 2,
            'B' | 'b' | 'C' | 'c' | 'M' | 'm' | 'P' | 'p' => score += 3,
            'F' | 'f' | 'H' | 'h' | 'V' | 'v' | 'W' | 'w' | 'Y' | 'y' => score += 4,
            'K' | 'k' => score += 5,
            'J' | 'j' | 'X' | 'x' => score += 8,
            'Q' | 'q' | 'Z' | 'z' => score += 10,
            _ => score += 0,
        };
    }

    score
}
