impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        /*
        iterate down the back of the string until you find space
        if its space return how many chars you passed
        */
        // &str is a borrowed reference to the string
        let words: Vec<&str> = s.split_whitespace().collect();
        // check in a null protected way
        // Some means that it has some value, a non None value
        if let Some(word) = words.last() {
            return word.len() as i32;
        }
        return 0;
    }
}