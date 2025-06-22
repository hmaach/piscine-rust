pub fn score(s: &str) -> u64 {
    let mut res = 0;

    for ch in s.chars() {
        match ch.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => res += 1,
            'F' | 'H' | 'V' | 'W' | 'Y' => res += 4,
            'B' | 'C' | 'M' | 'P' => res += 3,
            'Q' | 'Z' => res += 10,
            'D' | 'G' => res += 2,
            'J' | 'X' => res += 8,
            'K' => res += 5,
            _ => (),
        }
    }

    res
}
