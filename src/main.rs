#![forbid(unsafe_code)]

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if let Err(err) = jwt_decode::run(&args) {
        eprintln!("An error occurred: {:?}", err);
        std::process::exit(1);
    }
}
