use std::error::Error;
use std::fs;

pub enum Mode {
    Encrypt,
    Decrypt,
}

pub struct Config {
    pub key: i32,
    pub mode: Mode,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
           return Err("not enough arguments");
        }

        let key: i32 = args[1].trim().parse().expect("Please type a number");

        let mode = if args[2] == "encoding" {
            Mode::Encrypt
        } else  if args[2] == "decoding" {
            Mode::Decrypt
        } else {
            return Err("mode: \"encoding\" or \"decoding\"");
        };

        let file_path: String = args[3].clone();

        Ok(Config {
            key,
            mode,
            file_path
        })

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.file_path)?;

    let translated = match config.mode {
        Mode::Encrypt => {},
        Mode::Decrypt => {},
    }

}


pub fn translate_message(key: i32, content: &str) {

}