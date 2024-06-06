

// Given a string s, find the length of the longest
// substring
//  without repeating characters.
// Example 1:
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::cmp::max;
    use std::collections::{HashSet, VecDeque};
    // think the way to do this one is actually two pointer
    // sliding window
    // set of characters, on a match, pop until you find the one that matched
    let mut longest: i32 = 0;
    let mut deq: VecDeque<char> = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if !set.contains(&c) {
            // contains is always a ref
            deq.push_back(c);
            set.insert(c);
        } else {
            // a lot of the data structures will put something in an option
            while let Some(front_char) = deq.pop_front() {
                // remove each
                set.remove(&front_char);
                // match and stop
                if front_char == c {
                    break;
                }
            }
            // add the dupe
            deq.push_back(c);
            set.insert(c);
        }
        longest = max(longest, deq.len() as i32);
    }
    longest
}

pub fn length_of_longest_substring_optimal(s : String) -> i32 {
    // char of indicies 123 for each ascii character, initialize to -1
    let mut char_indices: Vec<i32> = vec![-1; 128]; // each char will map to one of these
    let mut start = 0; // index values are all -1 so start will be  greater at beginning
    let mut max_length = 0;
    // loop with indicies and character
    for (i, c) in s.char_indices() {
        let i = i as i32;
        start = start.max(char_indices[c as usize] + 1); // beginning: max between zero and -1 + 1 ...
        max_length = max_length.max(i - start + 1); // difference of index zero and one is a len of 2
        char_indices[c as usize] = i; // save the index of the last time you saw it, which will swap w start if we see again
        // > ... < this is a good algorithm
    }
    max_length
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::{length_of_longest_substring, length_of_longest_substring_optimal};

    #[test]
    pub fn length_of_longest_substring_test() {
        let mut s = "abcabcbb".to_string();
        assert_eq!(3, length_of_longest_substring(s));
        s = "bbbbb".to_string();
        assert_eq!(1, length_of_longest_substring(s));
        s = "pwwkew".to_string();
        assert_eq!(3, length_of_longest_substring(s));
        s = "aab".to_string();
        assert_eq!(2, length_of_longest_substring(s));
        s = "aab".to_string();
        assert_eq!(2, length_of_longest_substring(s));
        s = "tmmzuxt".to_string();
        assert_eq!(5, length_of_longest_substring(s));
    }

    #[test]
    pub fn length_of_longest_substring_optimal_test() {
        let mut s = "abcabcbb".to_string();
        assert_eq!(3, length_of_longest_substring_optimal(s));
        s = "bbbbb".to_string();
        assert_eq!(1, length_of_longest_substring_optimal(s));
        s = "pwwkew".to_string();
        assert_eq!(3, length_of_longest_substring_optimal(s));
        s = "aab".to_string();
        assert_eq!(2, length_of_longest_substring_optimal(s));
        s = "aab".to_string();
        assert_eq!(2, length_of_longest_substring_optimal(s));
        s = "tmmzuxt".to_string();
        assert_eq!(5, length_of_longest_substring_optimal(s));
    }
}
