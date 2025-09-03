use std::process;
use rand::Rng;
use rand::seq::SliceRandom;
use the_transposition_cipher::{decrypt_message, encrypt_message};

fn main() {

    let mut rng = rand::rng();

    for test_number in 1..=20 {
        let text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut message = String::new();

        let text_times = rng.random_range(4..40);

        for _ in 0..text_times {
            message.push_str(text);
        }

        let mut bytes = message.into_bytes();
        bytes.shuffle(&mut rng);
        let str = String::from_utf8(bytes).unwrap();

        println!("Test #{}: {}", test_number, &str[..50]);

        let max_key = str.len() / 2;

        for key in 1..max_key {
            let encrypted = encrypt_message(key, &str);
            let decrypted = decrypt_message(key, &encrypted);

            if str != decrypted {
                println!("Mismatch with key {} and message {}", key, str);
                process::exit(1);
            }
        }
    }
    println!("Transposition cipher test passed.");
}
