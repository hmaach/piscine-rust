#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let res: String = original
        .chars()
        .map(|ch| {
            if ch.is_ascii_uppercase() {
                (b'A' + (b'Z' - ch as u8)) as char
            } else if ch.is_ascii_lowercase() {
                (b'a' + (b'z' - ch as u8)) as char
            } else {
                ch
            }
        })
        .collect();

    if ciphered != res {
        return Err(CipherError { expected: res });
    }
    Ok(())
}
