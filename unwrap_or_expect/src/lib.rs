// Instructions
// Create a function named fetch_data with two arguments:

// server: A Result<String, String>, with either a server URL or an error message inside, respectively.
// security_level: An enum instance representing the desired behavior of the function in case of errors.
// The security_level enum should be defined as follows:

// Unknown: Returns the server URL or panics.
// Message: Returns the server URL or panics with the error message ERROR: program stops.
// Warning: Returns the server URL or the message WARNING: check the server.
// NotFound: Returns the server URL or the message Not found: [MESSAGE], where [MESSAGE] represents the server's error message.
// UnexpectedUrl: Returns the error message or panics with the error message being the server URL.

pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String  {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops"),
        Security::Warning => server.unwrap_or("WARNING: check the server"),
        Security::NotFound => server.unwrap_or_else(|err| format!("Not Found: {}", err)),
        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(err) => err.to_string(),
        },

    }
}
