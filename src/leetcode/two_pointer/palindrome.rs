

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // the chars thing is bs TODO: lookup char in Rust
        let cleaned: String = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        // edge cases to waste time
        if cleaned.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = cleaned.len() - 1;

        while left < right {
            if cleaned.chars().nth(left).unwrap() != cleaned.chars().nth(right).unwrap() {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}