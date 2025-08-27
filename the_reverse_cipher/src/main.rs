fn main() {
    let message = "Three can keep a secret, if two of them are dead";

    let mut translated = String::new();

    let mut i = message.len();

    while i > 0 {
        translated.push_str(&message[i-1..i]);

        i = i - 1;
    }

    println!("{}", translated);
}
