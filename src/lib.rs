//! # JWT Decoder
//!
//! `jwt` is a JSON Web Token decoder, which makes debugging with JWTs more convenient.

use std::error::Error;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    match parse_arguments(args)? {
        Some(argument) if argument != "-h" && argument != "--help" => {
            let decoded_jwt = process_jwt(argument)?;
            printout_decoded_jwt(&decoded_jwt);
        }
        _ => printout_help(),
    }

    Ok(())
}

/// Processes arguments given to `jwt`
///
/// # Examples
/// ```
/// let args = ["/bin/jwt".to_string(), "--help".to_string()];
///
/// assert_eq!(Ok(Some("--help")), jwt_decode::parse_arguments(&args))
/// ```
pub fn parse_arguments(args: &[String]) -> Result<Option<&str>, &'static str> {
    if args.len() == 0 {
        return Err("Program was called illegally!");
    }

    Ok(args.get(1).map(|s| &**s))
}

/// Processes a given JWT string by splitting it and decoding header and body
///
/// # Examples
///
/// ```
/// let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
///
/// assert_eq!(
///     vec![r#"{"alg":"HS256","typ":"JWT"}"#, r#"{"sub":"1234567890","name":"John Doe","iat":1516239022}"#],
///     jwt_decode::process_jwt(&jwt.to_string()).unwrap()
/// );
/// ```
pub fn process_jwt(jwt: &str) -> Result<[String; 2], Box<dyn Error>> {
    let splitted_jwt_strings: Vec<_> = jwt.split('.').collect();

    let jwt_header = splitted_jwt_strings.get(0).expect("split always returns at least one element");
    let jwt_body = splitted_jwt_strings.get(1).ok_or(Box::<dyn Error>::from("Could not find separator in jwt string."))?;

    let decoded_jwt_header = base64::decode(jwt_header)?;
    let decoded_jwt_body = base64::decode(jwt_body)?;

    let converted_jwt_header = String::from_utf8(decoded_jwt_header)?;
    let converted_jwt_body = String::from_utf8(decoded_jwt_body)?;

    Ok([converted_jwt_header, converted_jwt_body])
}

fn printout_decoded_jwt(jwt: &[String]) {
    println!("{}", jwt[0]);
    println!("{}", jwt[1]);
}

fn printout_help() {
    println!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("jwt displays a given base64 encoded JWT String in a readable manner.\n
        JWT header and body are printed in separate lines.\n");
    println!("Syntax: {} [Options] <JWT>\n", env!("CARGO_PKG_NAME"));
    println!("Options:\n\t--help,  -h\t View this help message");
}
