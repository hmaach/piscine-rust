#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut my_ciphered = String::new();

    for ch in original.to_string().chars() {
        if ch.is_uppercase() && ('Z' as u32 - ch as u32 + 'A' as u32 > 0) {
            my_ciphered.push(('Z' as u32 - ch as u32 + 'A' as u32) as u8 as char);
        } else if ch.is_lowercase() && ('z' as u32 - ch as u32 + 'a' as u32 > 0) {
            my_ciphered.push(('z' as u32 - ch as u32 + 'a' as u32) as u8 as char);
        } else {
            my_ciphered.push(ch);
        }
    }
    if ciphered != my_ciphered.as_str() {
        return Err(CipherError {
            expected: my_ciphered,
        });
    }
    Ok(())
}
