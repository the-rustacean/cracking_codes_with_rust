pub fn encrypt_message(key: usize, message: &str) -> String {

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

pub fn decrypt_message(key: usize, message: &str) -> String {

    let mut plain_text: Vec<String> = vec![];
    let message_length = message.len();
    let row_length = key;
    let column_length = (message_length / row_length) + 1;
    let rest_length = (row_length * column_length) - message_length;

    for column in 0usize..column_length {

        let mut current_index = column; // 0,1,2,3
        let mut column_text = String::new();
        let mut column_count = column_length;

        for row in 0usize..row_length { // 0,1,2,3,4,5|6,7

            if (current_index < message_length) && (column < column_length - 1 || (column == column_length - 1 && row < row_length - rest_length)) {
                column_text.push_str(&message[current_index..current_index + 1]);
            }

            if row == row_length - rest_length {
                column_count = column_length - 1;
            }

            current_index = current_index + column_count;
        }

        plain_text.push(column_text);
    }

    plain_text.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let message = "Common sense is not so common.";
        let key = 8;

        assert_eq!(encrypt_message(key, message), "Cenoonommstmme oo snnio. s s c");
    }

        #[test]
    fn test_decrypt() {
        let message = "Cenoonommstmme oo snnio. s s c";
        let key = 8;

        assert_eq!(decrypt_message(key, message), "Common sense is not so common.");
    }
}
