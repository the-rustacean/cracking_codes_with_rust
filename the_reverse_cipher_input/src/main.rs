use std::io;

fn main() {
    println!("Enter message:");

    let mut message = String::new();

    io::stdin().read_line(&mut message).expect("Failed to read line");

    let mut translated = String::new();

    let mut i = message.len();

    while i > 0 {
        translated.push_str(&message[i-1..i]);

        i = i - 1;
    }

    println!("{}", translated);
}
