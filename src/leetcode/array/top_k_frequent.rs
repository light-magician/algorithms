
use std::collections::{
    HashMap,
    BinaryHeap
};
use std::cmp::Reverse;

/*
The obvious answer is to count them and sort by frequency.
Answer two is to use a max heap or an inverted min heap if that
    is how heap is implemented in rust.
I think there is a more technical solution as well...

There is an O(n) solution
There is also a quickselect solution that is also O(n)
*/

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map : HashMap<i32, usize> = HashMap::new();
        for num in nums {
            // want to alter the value that entry(_) points to
            *map.entry(num).or_insert(0) +=1;
        }
        let mut heap : BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();
        // Push elements along with their frequency to the heap
        for (&num, &freq) in &map {
            heap.push(Reverse((freq, num)));
        }

        // Pop elements from the heap to get the most frequent ones
        let top5: Vec<i32> = heap
            .into_sorted_vec()
            .into_iter()
            .take(k as usize)
            .map(|Reverse((_, num))| num)
            .collect();
        return top5;
    }
}