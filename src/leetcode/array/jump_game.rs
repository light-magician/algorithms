impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        /*
        brute force would be to for each index we can reach
            dfs that the final index can be reached from there
            we can work backward from the final index and see
            lets make an array of boolean, and say can we
            reach another index from which the last index is reachable
            that would be O(n) because we just need to know if its within
            the correct distance. Easier than expected.
        // */
        // let can_reach_end: Vec<bool> = vec![false; nums.len()];
        // can_reach_end[nums] = true;
        // loop backwards, find if reaching next true is possible
        // track the closest true
        let mut closest_true_index = nums.len() - 1;
        for i in (0..nums.len() - 1).rev() {
            if (nums[i] as usize) + i >= closest_true_index {
                closest_true_index = i;
            }
        }
        return closest_true_index == 0;
    }
}