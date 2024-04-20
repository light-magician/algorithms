use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        /*
        this one is actually interesting
        we need to map each letter to a number and see if 

        can go O(n) memory O(N) runtime
        */
        let mut rep1: Vec<usize> = Vec::new();
        let mut rep2: Vec<usize> = Vec::new();
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut unique: usize = 0;
        for c in s.chars() {
            match map.get(&c) {
                Some(&val) => {
                    rep1.push(val);
                },
                None => {
                    rep1.push(unique);
                    map.insert(c, unique);
                    unique += 1;
                }
            }
        }
        map.clear();
        unique = 0;
        for c in t.chars() {
            match map.get(&c) {
                Some(&val) => { // 011
                    rep2.push(val);
                },
                None => {
                    rep2.push(unique);
                    map.insert(c, unique);
                    unique += 1;
                }
            }
        }
        return rep1 == rep2;
    }
}