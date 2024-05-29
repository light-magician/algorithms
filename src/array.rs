pub mod array {

    // Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
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
        // get counts, make vec of deque for each counts, loop backwards pop until empty then decrement
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
}

#[cfg(test)]
mod array_test {
    use super::*;
    use array::{top_k_frequent_n, top_k_frequent_nlogn};

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
