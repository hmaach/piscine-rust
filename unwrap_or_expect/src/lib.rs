pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}
use Security::*;

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Unknown => server.unwrap().to_string(),
        Message => server.expect("ERROR: program stops").to_string(),
        Warning => server.unwrap_or("WARNING: check the server").to_string(),
        NotFound => server.map(String::from).unwrap_or_else(|f| format!("Not found: {}" , f)),
        _ => server.unwrap_err().to_string(),
    }
}