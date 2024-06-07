

// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

 

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:

// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut forward: Vec<i32> = Vec::new();
        let mut sum = 1;
        for num in nums.clone() {
            sum = sum * num;
            forward.push(sum);
        }
        sum = 1;
        let mut backward: Vec<i32> = Vec::new();
        for &num in nums.iter().rev() { // too many loop variants
            sum = sum * num;
            backward.push(sum)
        }
        // cant add to front unless deque so have to reverse backward
        backward.reverse();
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            if i == 0 {
                ans.push(backward[i + 1]);
            } else if i == nums.len() - 1 {
                ans.push(forward[i - 1]);
            } else {
                ans.push(backward[i  + 1] * forward[i - 1])
            }
        }

        return ans;
}


// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
// You must write an algorithm that runs in O(n) time.
// Example 1:
// Input: nums = [100,4,200,1,3,2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
// Example 2:
// Input: nums = [0,3,7,2,5,8,4,6,0,1]
// Output: 9
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut longest = 0;
    if nums.len() == 0 {return longest}
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.into_iter().collect();
    // this will only search starting with each starting point, clever
    for &num in &set {
        // potential start of sequence?
        if !set.contains(&(num - 1)) {
            let mut curr = num;
            let mut seq_len = 1;
            while set.contains(&(curr + 1)) {
                curr += 1;
                seq_len += 1;
            }
            longest = longest.max(seq_len);
        }
    }
    longest
}

#[cfg(test)]
pub mod array_tests {

    use crate::arrays::*;

    #[test]
    pub fn longest_consecutive_test() {
        let mut nums = vec![100,4,200,1,3,2];
        assert_eq!(4, longest_consecutive(nums));
        nums = vec![0,3,7,2,5,8,4,6,0,1];
        assert_eq!(9, longest_consecutive(nums));
    }

    #[test]
    pub fn product_array_except_self_test() {
        let mut nums = vec![1,2,3,4];
        let mut output = vec![24,12,8,6];
        assert_eq!(output, product_except_self(nums));
        nums = vec![-1,1,0,-3,3];
        output = vec![0,0,9,0,0];
        assert_eq!(output, product_except_self(nums));
    }
}