impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // this can also be one with arrays and bit mapping or whatever, mem saver
        let mut mapS: HashMap<char, i32> = HashMap::new();
        let mut mapT: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *mapS.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *mapT.entry(c).or_insert(0) += 1;
        }
        return mapS == mapT;
    }
}