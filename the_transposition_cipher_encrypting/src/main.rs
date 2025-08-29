fn main() {
    let message = "Common sense is not so common.";

    let key = 8;

    let cipher_text = encrypt_message(key, message);

    println!("{}|", cipher_text);
}

fn encrypt_message(key: usize, message: &str) -> String {

    let mut cipher_text: Vec<String> = vec![];

    for column in 0usize..key {

        let mut current_index = column;
        let mut column_text = String::new();

        while current_index < message.len() {
            column_text.push_str(&message[current_index..current_index + 1]);
            current_index = current_index + key;
        }

        cipher_text.push(column_text);
    }

    cipher_text.join("")
}
