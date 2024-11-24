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
    for &num in nums.iter().rev() {
        // too many loop variants
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
            ans.push(backward[i + 1] * forward[i - 1])
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
    if nums.len() == 0 {
        return longest;
    }
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

pub fn max_profit(prices: Vec<i32>) -> i32 {
    // at first I bought high and sold low XD
    // keep track of the lowest price seen, and max profit recorded
    let mut low = i32::MAX;
    let mut profit = 0;
    for price in prices {
        // either price is lower than min price or check for profit
        if price < low {
            low = price;
        }
        profit = profit.max(price - low);
    }
    return profit;
}

// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, 
// and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
// The final sorted array should not be returned by the function, but instead be stored inside the array nums1.
// To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged,
// and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    /*
    We are putting all the zeros in the second array, we only increment if we swap with a zero
        when we swap with a number finally
    pointer at the start of each array
    check for least between them, 
        whichever is least, take that one
    We need a pointer to the insertion index and it goes backwards
        that would be a useize of m - 1
    Run until insert_index is -1
    choose between list one and two, and swap the lesser with insert_index
    if the 
    Track boolean pivot thats where we want to decrament our a1 now rather than increment
        we pivot when we swap two numbers that are not zero from a1 and a2
        I think it is when our reverse index is not at a zero anymore
        a1 ptr will decrease now, a2 pointer will not move
    If you swapp and its not a zero at ins_idx pivot = true
    */
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut ins_idx: i32 = m - 1;
    let mut pivot: bool = false;
    let mut just_pivoted: bool = false;
    while ins_idx >= 0 {
        let ins_idx_size: usize = ins_idx as usize;
        // decide which is less and swap and increment unless pivot
        if nums1[ins_idx_size] != 0 {
            pivot = true;
            just_pivoted = true;
        }

        let num1 = nums1[p1];
        let num2 = nums2[p2];
        // you did not code to swap it if measured against zero

        if nums1 <= nums2 {
            let temp: i32 = nums1[p1];
            nums1[p1] = nums1[ins_idx_size];
            nums1[ins_idx_size] = temp;
            if pivot {
                p1 -= 1;
            } else {
                p1 += 1;
            }
        } else {
            let temp: i32 = nums2[p2];
            nums2[p2] = nums1[ins_idx_size];
            nums1[ins_idx_size] = temp;
            if just_pivoted {
                just_pivoted = false;
                break;
            }
            p2 += 1;
        }
        // always
        ins_idx -= 1;
    }


}

#[cfg(test)]
pub mod array_tests {

    use crate::arrays::*;

    #[test]
    pub fn merge_sorted_array_test() {
        let mut a1: Vec<i32> = vec![1, 3, 6, 8, 10, 20, 25, 27, 0, 0, 0, 0, 0, 0, 0, 0];
        let a1_len: i32 = a1.len() as i32;
        let mut a2: Vec<i32> = vec![1, 2, 3, 7, 14, 21, 23, 29];
        let a2_len: i32 = a2.len() as i32;
        // 0, 0, 4, 3, 2, 1
        // 0, 5, 6
        // if you hit a zero favor the nuber
        let ans: Vec<i32> = vec![1, 1, 2, 3, 3, 6, 7, 8, 10, 14, 20, 21, 23, 25, 27, 29];
        merge_sorted_array(&mut a1, a1_len, &mut a2, a2_len);
        assert_eq!(a1, ans);
    }

    #[test]
    pub fn max_profit_test() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, max_profit(nums));
    }

    #[test]
    pub fn longest_consecutive_test() {
        let mut nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(4, longest_consecutive(nums));
        nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(9, longest_consecutive(nums));
    }

    #[test]
    pub fn product_array_except_self_test() {
        let mut nums = vec![1, 2, 3, 4];
        let mut output = vec![24, 12, 8, 6];
        assert_eq!(output, product_except_self(nums));
        nums = vec![-1, 1, 0, -3, 3];
        output = vec![0, 0, 9, 0, 0];
        assert_eq!(output, product_except_self(nums));
    }
}
