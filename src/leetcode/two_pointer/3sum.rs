use std::collections::{HashMap, HashSet}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        /*
        whats the caveat, not so easy this time just to two pointer it
        a hashtable is also less likely useful because there are numerous repeats
        if the list is sorted, we at least know which direction to look for 
        the third in, nlog(n) plus an n squared.
        In a sense, this begs the question as to whether or not
        you can efficiently do this for any number combo
        we dont need to know the original number index
        we can make a set of sets situation where the number and its indexes are blah blah
            num to index match set
        thats a lot of extra memory
        requires a sort as well
        lets try that
        */

        // let mut map: HashMap<i32, HashSet<usize>>. = HashMap::new();
        // // have to get good at using HashMaps and Sets in rust
        // for i in 0..nums.len() {
        //     let mut num = nums[i]; // an i32
        //     map.entry(num).or_insert_with(HashSet::new()).insert();
        //     // that gives the proper hashset, now the two pointer approach
        // }
        // // vec.sort sorts in place
        let mut l: usize = 0;
        let mut  r: usize = nums.len() - 1;
        // loop get each sum, find in the set if there is an index that is not in there
        /*
        no longer feeling like a good solution really
        we have to efficiently check that teh antecedent number is not already in use
        one way to check that is to see if it matches l or r, or both, then we just need to know
        if the Hashset is longer, so we may not need the set
        we can just sort the list and do an o(n^2) and save ourselves some brain cells
        */
        nums.sort();
        /*
        oh shit can you binary sort this out once its sorted
        O(n*log(n)) to sort
        O(n*log(n)) same thing to search out appropriate numbers, every number is either
            too large or too small, maybe come back to this one once we have some binary search practice
        */
    }
}