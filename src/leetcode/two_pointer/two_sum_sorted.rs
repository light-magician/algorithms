impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        return: [index1, index2] not just index, indexes cast to i32

        approach:
            two pointer on l and r
            if its too large decrement r
            too small increment l
        */
        let mut l: usize = 0;
        let mut r: usize = numbers.len() - 1;
        while l < r {
            let seek: i32 = numbers[l] + numbers[r];
            if seek > target {
                r -= 1;
            } else if seek < target {
                l += 1;
            } else {
                // increment both indexes as per directions
                l += 1;
                r += 1;
                return vec![l as i32, r as i32];
            }
        }
        vec![] // this isnt python so some type of guaranteed return is expected
    }
}