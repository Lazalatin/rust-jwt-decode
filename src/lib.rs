//! # JWT Decoder
//!
//! `jwt` is a JSON Web Token decoder, which makes debugging with JWTs more convenient.

use base64;
use std;

pub fn run(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let jwt_string = parse_arguments(args).unwrap();

    if jwt_string == "-h" || jwt_string == "--help" {
        printout_help();
        std::process::exit(0);
    } else {
        let decoded_jwt: Vec<String> = process_jwt(jwt_string).expect("Something went wrong parsing the jwt");
        printout_decoded_jwt(&decoded_jwt);
        std::process::exit(0);
    }
}

/// Processes arguments given to `jwt`
///
/// # Examples
/// ```
/// let args: &Vec<String> = &vec!("/bin/jwt".to_string(), "--help".to_string());
///
/// assert_eq!(Ok("--help"), jwt::parse_arguments(&args))
/// ```
pub fn parse_arguments(args: &Vec<String>) -> Result<&str, &'static str> {
    if args.len() == 0 {
        return Err("Program was called illegally!");
    } else if args.len() == 1 {
        return Ok("--help");
    } else {
        let result: &str = &args[1];
        return Ok(result);
    }
}

/// Processes a given JWT string by splitting it and decoding header and body
///
/// # Examples
///
/// ```
/// let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
///
/// assert_eq!(vec!["{\"alg\":\"HS256\",\"typ\":\"JWT\"}", "{\"sub\":\"1234567890\",\"name\":\"John Doe\",\"iat\":1516239022}"], jwt::process_jwt(&jwt.to_string()).unwrap());
/// ```
///
/// # Panic
/// This function may panic if the given string is not processable
///
pub fn process_jwt(jwt: &str) -> Result<Vec<String>, &'static str> {
    let splitted_jwt_strings: Vec<&str> = jwt.split('.').collect();

    let jwt_header = splitted_jwt_strings[0];
    let jwt_body = splitted_jwt_strings[1];

    let decoded_jwt_header = base64::decode(jwt_header).unwrap();
    let decoded_jwt_body = base64::decode(jwt_body).unwrap();

    let converted_jwt_header = String::from_utf8(decoded_jwt_header).unwrap();
    let converted_jwt_body = String::from_utf8(decoded_jwt_body).unwrap();

    return Ok([converted_jwt_header, converted_jwt_body].to_vec());
}

fn printout_decoded_jwt(jwt: &Vec<String>) {
    println!("{}", jwt[0]);
    println!("{}", jwt[1]);
}

fn printout_help() {
    println!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("jwt displays a given base64 encoded JWT String in a readable manner.\nJWT header and body are printed in separate lines.\n");
    println!("Syntax: {} [Options] <JWT>\n", env!("CARGO_PKG_NAME"));
    println!("Options:\n\t--help,  -h\t View this help message");
}
