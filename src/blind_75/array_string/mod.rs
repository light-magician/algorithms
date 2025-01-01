/// You are given two strings word1 and word2.
/// Merge the strings by adding letters in alternating order,
/// starting with word1. If a string is longer than the other,
/// append the additional letters onto the end of the merged string.
/// Return the merged string.
pub fn merge_alternately(word1: String, word2: String) -> String {
    let len1 = word1.len();
    let len2 = word2.len();
    let lower_len = len1.min(len2);
    let mut ans = String::new();

    for i in 0..lower_len {
        ans.push(word1.chars().nth(i).unwrap());
        ans.push(word2.chars().nth(i).unwrap());
    }

    let longer = if len1 > len2 { &word1 } else { &word2 };
    let diff = (len1 as i32 - len2 as i32).abs() as usize;

    if diff > 0 {
        ans.push_str(&longer[lower_len..]);
    }
    ans
}

/// For two strings s and t, we say "t divides s"
/// if and only if s = t + t + t + ... + t + t
/// (i.e., t is concatenated with itself one or more times).
/// Given two strings str1 and str2, return the largest string
/// x such that x divides both str1 and str2.
pub fn gcd_of_strings(str1: String, str2: String) -> String {}

#[cfg(test)]
mod array_string_tests {

    use super::*;

    #[test]
    fn test_merge_alternately() {
        let mut word1 = String::from("abc");
        let mut word2 = String::from("hello");
        assert_eq!("ahbecllo", merge_alternately(word1, word2));

        word1 = String::from("!@#$%^&*");
        word2 = String::from("");
        assert_eq!("!@#$%^&*", merge_alternately(word1, word2));
    }
}
