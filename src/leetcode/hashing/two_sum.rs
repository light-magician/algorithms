use std::collections::{HashMap}; //have to close these

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let num = nums[i];
            let other: i32 = target - nums[i];
            if map.contains_key(&other) { //need to learn about how some things are borrowed and some are not
                let second_i: usize = *map.get(&other).unwrap_or(&0);
                return vec![i as i32, second_i as i32];
            } else {
                map.insert(num, i);
            }
        }
        // not python must imply something is returned
        vec![]
    }
}