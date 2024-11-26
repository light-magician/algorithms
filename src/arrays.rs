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

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

/// takes a list of nums
/// returns length of list after dupe removal
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // loop from one to end
    let mut i: usize = 1;
    while i < nums.len() {
        // only increment if not removed
        let curr = nums[i];
        let prev = nums[i - 1];
        if curr == prev {
            nums.remove(i);
            continue;
        }
        i += 1;
    }

    nums.len() as i32
}

// remove so that each element appears at most twice
// extra space allocation is not permitted
pub fn remove_duplicates_two(nums: &mut Vec<i32>) -> i32 {
    // whats the simplest way to do this, expand mem with a map
    // lead and tail pointer  if its a new value
    // when we find a new one, delete whats betwen current and tail
    // then reset curr and tail
    let mut lead: usize = 0;
    let mut tail: usize = 0;
    while lead < nums.len() {
        if nums[lead] != nums[tail] {
            // 1, 1, 1, 2, 2, 2
            // 0, 1, 1, 2, 2
            // whats the diff, you want it to be 2 as in 0, 1, 2
            while lead - tail > 2 {
                nums.remove(lead - 1);
                lead -= 1; // move it back in
            }
            tail = lead;
        } else {
            lead += 1;
        }
    }
    // risky removing on the tail index
    if lead - tail > 2 {
        while lead - tail > 2 {
            nums.remove(lead - 1);
            lead -= 1; // move it back in
        }
    }
    // handle the edge case here
    // 1 1 2 2 2 x
    nums.len() as i32
}

// Input: nums = [1,2,3,4,5,6,7], k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// 1 2 3 4 5 6
// rotate 1 steps to the right: [7,1,2,3,4,5,6]
// rotate 2 steps to the right: [6,7,1,2,3,4,5]
// rotate 3 steps to the right: [5,6,7,1,2,3,4]
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    // what I am thinking is st ptr at zero and len - k
    // rotate the right number of times, always n
    // if head ptr is len wrap it to zero

    
    let head: usize = nums.len() - 1 - k as usize - 1;
    let base: usize = 0;
    /// there were 6 elements and we flipped 2, meaning we need to flip 4 more
    /// meaning we need 2 more flips, but where should the indexes be. 
    /// base can stay the same, and it should flip with the index we started flipping at
    /// perform k flips, then from base and original head, perform len - 1 - k flips ?
    /// len 6, k = 2 , do 2 flips 
    // this is actually a good question
    // once you have done k flips its done if its even
    // if its odd you have to do n flips, if even you have to do 
    let mut ans: Vec<i32> =Vec::new();
    let mut seek: usize = nums.len() - k as usize; // should yeild correct result
    let mut count: usize = 0;
    while count < nums.len() {
        let num = nums[seek];
        ans.push(num);
        seek += 1;
        count += 1;
        if seek == nums.len() {
            seek = 0;
        }
    }
    for i in 0..nums.len() {
        nums[i] = ans[i];
    }
}

#[cfg(test)]
pub mod array_tests {

    use crate::arrays::*;

    #[test]
    pub fn remove_dupliicates_two() {
        let mut arr: Vec<i32> = vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
        let ans: i32 = 6;
        assert_eq!(remove_duplicates_two(& mut arr), ans);
    }

    #[test]
    pub fn remove_duplicate() {
        let mut arr: Vec<i32> = vec![1, 1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5];
        let ans: i32 = 5;
        assert_eq!(remove_duplicates(& mut arr), ans);
    }

    #[test]
    pub fn remove_element_test() {
        //remove an element in place
        let mut arr: Vec<i32> = vec![1, 2, 3, 5, 5, 6, 7, 1, 5];
        let num: i32 = 5;
        let ans: Vec<i32> = vec![1, 2, 3, 6, 7, 1];
        assert_eq!(remove_element(&mut arr, num), ans.len() as i32);
    }

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
