use std::env;
use std::process;

use the_transposition_cipher_file::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Program parsing arguments: {err}");
        process::exit(1);
    });

    // println!("mode: {}", config.mode);
    // println!("file: {}", config.file_path);

    if let Err(e) = the_transposition_cipher_file::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
