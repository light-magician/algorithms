

pub mod Palindrome {
    /*
    A phrase is a palindrome if, after converting all uppercase 
    letters into lowercase letters and removing all non-alphanumeric 
    characters, it reads the same forward and backward. 
    Alphanumeric characters include letters and numbers.
     */
    pub fn valid_palindrome(s: String) -> bool {
        // convert a String to a char vec
        let chars: Vec<char> = s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

        let mut left = 0;
        let mut right = chars.len() -1;

        while left < right {
            let left_char =  chars[left];
            let right_char = chars[right];
            if left_char != right_char {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_valid_palindrome() {
            assert_eq!(valid_palindrome(" ".to_string()), true);
        }
    }
}