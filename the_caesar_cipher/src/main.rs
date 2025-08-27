fn main() {
    let message = "This is my secret message.";
    // let message = "guv6Jv6Jz!J6rp5r7Jzr66ntrM";

    let key = 13;

    let mode = Mode::Encrypt;

    const SYMBOLS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    let len = SYMBOLS.len();

    let mut translated = String::new();

    for symbol in message.chars() {
        match SYMBOLS.find(symbol) {
            Some(index) => {

                let translated_index;

                match mode {
                    Mode::Encrypt => {
                        translated_index = if index + key < len {
                            index + key
                        } else {
                            index + key - len
                        };
                    },
                    Mode::Decrypt => {
                        translated_index = match index.checked_sub(key) {
                            Some(i) => i,
                            None => len + index - key,
                        }
                    },
                }

                translated.push_str(&SYMBOLS[translated_index..translated_index+1]);
            },
            None => {
                translated.push(symbol);
            },
        }

    }

    println!("{}", translated);
}

enum Mode {
    Encrypt,
    Decrypt,
}
