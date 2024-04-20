use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // need to know how to split or something
        let mut pattern_nums: Vec<usize> = Vec::new();
        let mut pattern_map: HashMap<char, usize> = HashMap::new();
        let mut s_nums: Vec<usize> = Vec::new();
        let mut s_map: HashMap<&str, usize> = HashMap::new();
        let mut unique: usize = 0;
        for c in pattern.chars() {
            match pattern_map.get(&c) { // take ref but not responsible
                Some(&val) => {
                    pattern_nums.push(val);
                },
                None => {
                    pattern_nums.push(unique);
                    pattern_map.insert(c, unique);
                    unique += 1;
                }
            }
        }
        unique = 0;
        for token in s.split_whitespace() { // gives a string slice &str
            match s_map.get(&token) { // take ref but not responsible
                Some(&val) => {
                    s_nums.push(val);
                },
                None => {
                    s_nums.push(unique);
                    s_map.insert(token, unique);
                    unique += 1;
                }
            }
        }
        return pattern_nums == s_nums;
    }
}