use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        key number value index
        */
        let mut map = HashMap::new();
        // &num will get the i32, else it would be a pointer
        for (index, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&complement_index) = map.get(&diff) {
                return vec![complement_index as i32, index as i32];
            }
            map.insert(num, index);
        }
        vec![0, 0]
    }
}