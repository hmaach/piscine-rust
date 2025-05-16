pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.to_string().chars().count() == 0 {
        return Err("ERROR: illegal");
    }
    if message.to_string().contains("stupid") {
        return Err("ERROR: illegal");
    }
    Ok(message)
}
