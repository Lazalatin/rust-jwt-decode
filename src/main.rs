#![forbid(unsafe_code)]
use std;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(err) = jwt_decode::run(&args) {
        eprintln!("An Error occurred: {}", err.to_string());
        std::process::exit(1)
    }
}