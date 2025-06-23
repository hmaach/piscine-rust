use unwrap_or_expect::*;

fn main() {
    println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
    println!("{}", fetch_data(Err("server.com"), Security::Warning));
    println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

    // Panics with no custom message
    // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    // fetch_data(Err("server.com"), Security::Message);

    // Panics with the message "malicious_server.com"
    // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
}