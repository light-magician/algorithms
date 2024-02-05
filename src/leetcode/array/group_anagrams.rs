use std::collections::HashMap;

impl Solution {
    /*
    you can read the letters into a hashmap
    then make the hashmap a key
    read each vector into a counter key for another hashmap
    or you could sort them all.

    Most optimal solution for the key is to declare a fixed size array of 26
    constant size arrays are hashable
     */
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for string in strs {

            // fixed sized array of len 26 that is filled with zeros
            let mut alpha = [0_u8; 26]; // unsigned 8bit integers will fit

            // mem location minus 'a' character as a usize will give 0 -> 25
            for c in string.chars() {
                alpha[c as usize - 'a' as usize] += 1;
            }

            // if its in there push it if not insert it
            if let Some(array) = map.get_mut(&alpha) {
                array.push(string);
            } else {
                map.insert(alpha, vec![string]);
            }
        }
        map.into_values().collect()
    }
}