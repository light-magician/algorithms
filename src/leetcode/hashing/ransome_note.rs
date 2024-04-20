use std::collections::HashMap;

impl Solution { // lets get it with lower memory
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            // either get and increment by one or set zero get and increment by one
            *map.entry(c).or_insert(0) += 1; 
        }
        for c in ransom_note.chars() {
            *map.entry(c).or_insert(0) -= 1;
            match map.get(&c) {
                Some(&val) => { // need better intuition around borrowing
                    if val < 0 {
                        return false;
                    }
                },
                None => return false
            }
        }
        true
    }
}