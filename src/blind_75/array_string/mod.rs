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
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    // Start with the shorter string as potential GCD
    let mut gcd = if str1.len() <= str2.len() {
        str1.clone()
    } else {
        str2.clone()
    };
    
    // Loop through possible GCD lengths
    while !gcd.is_empty() {
        if string_is_divided(&str1, &gcd) 
        && string_is_divided(&str2, &gcd) {
            return gcd; 
        }
        // Remove last character to try shorter potential GCD
        gcd.truncate(gcd.len() - 1);
    }
    
    // Return empty string if no common divisor found
    String::new()
}

fn string_is_divided(strx: &String, test: &String) -> bool {
    // Check if the string length is divisible by test length
    if strx.len() % test.len() != 0 {
        return false;
    }
    
    // Check if the test string repeats perfectly to form strx
    for i in (0..strx.len()).step_by(test.len()) {
        let slice = String::from(&strx[i..i+test.len()]);
        if &slice != test {
            return false;
        }
    }
    
    true
}

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

    #[test]
    fn test_gcd_of_string() {
        let test_cases = [
            (String::from("ABCABC"), String::from("ABC"), String::from("ABC")),
            (String::from("ABABAB"), String::from("AB"), String::from("AB")),
            (String::from("TAUXXTAUXXTAUXXTAUXXTAUXX"), String::from("TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX"), String::from("TAUXX")),
        ];
        
        for (str1, str2, ans) in test_cases.iter() {
            assert_eq!(gcd_of_strings(str1.clone(), str2.clone()), ans.clone());
        }
    }
}
