impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    /*
    0 1 2 3 - 2
    should get 2 3 0 1
    learn that reverse syntax and that slicing syntax
    */
    // edge case of k = 0
    let n = nums.len() as i32;
    let k = k % n; // turn k into k mod len of nums to get wrapped k

    // Reverse the entire array
    nums.reverse(); // 3 2 1 0

    // Reverse the first k elements
    nums[..k as usize].reverse(); // 2 3 1 0

    // Reverse the remaining elements
    nums[k as usize..].reverse(); // 2 3 0 1
    }
}