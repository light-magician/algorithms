

impl Solution {

    use std::collections::HashMap;

    fn byte_pair_encoding(input_str: &str) -> String {
        // Example string
        let input_str = "hello world";

        // Convert string to bytes and record highest and lowest value bytes
        let mut input_bytes: Vec<u8> = input_str.bytes().collect();
        let (mut highest_byte, mut lowest_byte) = (0u8, std::u8::MAX);

        for &byte in &input_bytes {
            highest_byte = highest_byte.max(byte);
            lowest_byte = lowest_byte.min(byte);
        }

        // Loop through Vec of bytes two by two and create HashMap
        let mut byte_pair_counts = HashMap::new();
        let mut i = 0;
        while i < input_bytes.len() - 1 {
            let byte_pair = (input_bytes[i], input_bytes[i + 1]);
            *byte_pair_counts.entry(byte_pair).or_insert(0) += 1;
            i += 2;
        }

        // Efficiently modify the Vec in place
        while !byte_pair_counts.is_empty() {
            // Find the most common byte pair
            let (max_byte_pair, _) = byte_pair_counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .expect("HashMap is not expected to be empty");

            // Find a new byte that was not in the original byte array
            let new_byte = (highest_byte + 1) % (std::u8::MAX - lowest_byte + 1) + lowest_byte;

            // Replace occurrences of the most common byte pair
            i = 0;
            while i < input_bytes.len() - 1 {
                let byte_pair = (input_bytes[i], input_bytes[i + 1]);
                if byte_pair == *max_byte_pair {
                    input_bytes[i] = new_byte;
                    input_bytes[i + 1] = new_byte;
                }
                i += 2;
            }

            // Update HashMap and increment highest_byte
            byte_pair_counts.remove(max_byte_pair);
            highest_byte = new_byte;
        }

        // Turn the byte array back to a string and print it
        let result_str = String::from_utf8_lossy(&input_bytes);
        println!("{}", result_str);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_byte_pair_encoding() {
            // Shakespeare quote from Hamlet
            let hamlet_quote = "To be, or not to be, that is the question.";

            // Perform byte pair encoding
            let encoded_result = byte_pair_encoding(hamlet_quote);

            // Print the result
            println!("Original: {}", hamlet_quote);
            println!("Encoded : {}", encoded_result);

            // Add assertions based on your expectations
            // For example, you can check that the result is not equal to the original string
            assert_ne!(encoded_result, hamlet_quote);
        }
    }
}