impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        /*
        abs of iterator distance has to be < k to be valid
        I suspect the way to do this is to have a map,
        and always leave in the newest one that you find.
        */
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        for index in 0..nums.len() {
            let num: i32 = nums[index];
            match map.get(&num) {
                Some(val) => {
                    // does that mean abs is a 
                    if (*val as i32 - index as i32).abs() <= k {
                        return true;
                    }
                    if val < &index {
                        // move the current ahead
                        map.insert(num, index);
                    }
                },
                None => {
                    map.insert(num, index);
                }
            }
        }
        false
    }
}