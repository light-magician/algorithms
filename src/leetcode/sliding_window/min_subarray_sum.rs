impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        /*
            find a subarray that is greater than or equal to the sum
            this at the first level sounds like a backtracking problem
            each problem breaks down into another
            if target is 7 and I get a 3

            Sliding window:
                you can have a start and end, search until you are over,
                subtract until you are under
                the subarry does have to be contiguous so no sorting
        */
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        let mut min_size: i32 = 0;
        let mut curr_sum: i32 = 0;
        // O(n) search, grow until large enough, shrink until we can resize
        /*
        what are the conditions here, need to make sure the pointers dont overlap
        its also a good question when you want to tally.
         */
        while l < nums.len() && r < nums.len() {
            if curr_sum < target {
                curr_sum += nums[fast];
                fast += 1;
            } // shoot we have to think about the damn tracking, will work that out
        }
        return min_size;
    }  
}