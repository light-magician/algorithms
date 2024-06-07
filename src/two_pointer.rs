
// Given an integer array nums and an integer k, return the k
// most frequent elements. You may return the answer in any order.
// Example 1:
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]
// Example 2:
// Input: nums = [1], k = 1
// Output: [1]
pub fn top_k_frequent_nlogn(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // O(nlogn) version
    use std::collections::HashMap;
    let mut counts: HashMap<i32, i32> = HashMap::new();
    nums.into_iter().for_each(|x| {
        // get val at key, if you cant insert 0 at that key and increment
        *counts.entry(x).or_insert(0) += 1;
    });
    // get keys and values as a sorted list
    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    // iterating through collections frequently involves these closures
    sorted_counts.sort_by(|(_, a), (_, b)| b.cmp(a));
    return sorted_counts
        .into_iter()
        .take(k as usize)
        .map(|(x, _)| x)
        .collect();
}

pub fn top_k_frequent_n(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // O(n) version
    // get counts, make vec of deque for each counts, loop backwards
    // pop until empty then decrement
    use std::collections::{HashMap, VecDeque};

    let mut counts: HashMap<i32, i32> = HashMap::new();
    let size = nums.len();
    let capacity = k as usize;
    for i in nums {
        *counts.entry(i).or_insert(0) += 1;
    }
    // vector  or deques, each index represents a count, size + 1 bc len is 1 base
    let mut buckets: Vec<VecDeque<i32>> = vec![VecDeque::new(); size + 1];
    for (value, count) in counts {
        buckets[count as usize].push_back(value);
    }

    // collect the top k elements from the buckets
    let mut top_k: Vec<i32> = Vec::with_capacity(capacity);
    // loop down to 1 from len, pop until you get k elements
    for i in (1..buckets.len()).rev() {
        // pop until there is not Some value with a while loop
        while let Some(value) = buckets[i].pop_front() {
            top_k.push(value);
            if top_k.len() == capacity {
                return top_k;
            }
        }
    }
    top_k
}

// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
// Notice that the solution set must not contain duplicate triplets.
// Example 1:
// Input: nums = [-1,0,1,2,-1,-4]
//               [-4, -1, -1, 0 , 1, 2]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
// Example 2:
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
// Example 3:
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    let mut answer: Vec<Vec<i32>> = Vec::new();
    if nums.len() < 3 {return answer}
    let mut nums = nums;
    nums.sort();
    // from 1 to end - 1
    let mut center: usize = 1; // len 3 is 0 1 2 ...
    while center < nums.len() - 1 {
        let mut high:usize = nums.len() - 1;
        let mut low:usize = 0;
        /*
        there can be duplicates
         */
        while low != center && high != center {
            let sum: i32 = nums[low] + nums[center] + nums[high];
            if sum == 0 {
                let matches: Vec<i32> = vec![nums[low], nums[center], nums[high]];
                if !set.contains(&matches) { // no duplicate number entries
                    answer.push(matches.clone()); // have to clone or val is moved to arr
                    set.insert(matches);
                }
                // move in the one that is farthest away
                let left_distance: i32 = center as i32 - low as i32;
                let right_distance: i32 = high as i32 - center as i32;
                if left_distance > right_distance {
                    low += 1;
                } else {
                    high -= 1;
                }
            } else if sum > 0 {
                high -= 1;
            } else {
                low += 1;
            }
        }
        center += 1;
    }
    return answer;
}

#[cfg(test)]
mod array_test {
    use crate::two_pointer::{
        top_k_frequent_n, 
        top_k_frequent_nlogn,
        three_sum
    };

    #[test]
    pub fn three_sum_test() {

// Input: nums = [-1,0,1,2,-1,-4]
//               [-4, -1, -1, 0 , 1, 2]
// Output: [[-1,-1,2],[-1,0,1]]
        let mut nums: Vec<i32> = vec![-1,0,1,2,-1,-4];
        let mut answer: Vec<Vec<i32>> = vec![vec![-1,-1,2],vec![-1,0,1]];
        assert_eq!(answer, three_sum(nums));
        nums = vec![0, 0, 0, 0, 0];
        answer = vec![vec![0, 0, 0]];
        assert_eq!(answer, three_sum(nums));
        nums = vec![0, 1, 1];
        answer = vec![];
        assert_eq!(answer, three_sum(nums));
    }

    #[test]
    pub fn top_k_frequent_test_nlogn() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(vec![1, 2], top_k_frequent_nlogn(nums, k));
    }

    #[test]
    pub fn top_k_frequent_test_n() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let mut k = 2;
        assert_eq!(vec![1, 2], top_k_frequent_n(nums, k));
        nums = vec![1];
        k = 1;
        assert_eq!(vec![1], top_k_frequent_n(nums, k));
        nums = vec![-1, -1];
        k = 1;
        assert_eq!(vec![-1], top_k_frequent_n(nums, k));
    }
}
