fn main() {
    let message = "guv6Jv6Jz!J6rp5r7Jzr66ntrM";

    const SYMBOLS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

    let len = SYMBOLS.len();

    for key in 0..SYMBOLS.len() {

        let mut translated = String::new();

        for symbol in message.chars() {

            match SYMBOLS.find(symbol) {
                Some(index) => {

                    let translated_index;

                    translated_index = match index.checked_sub(key) {
                        Some(i) => i,
                        None => len + index - key,
                    };

                    translated.push_str(&SYMBOLS[translated_index..translated_index+1]);
                },
                None => {
                    translated.push(symbol);
                },
            }
        }

        println!("Key #{}: {}", key, translated);
    }
}
