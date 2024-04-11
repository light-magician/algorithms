impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        /*
        get a right sum array
        and a left sum array
        sum of array except self if sum of 
        whats on the right and left of current
        */
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        let mut sum = 1;
        for i in 0..nums.len() {
            sum = sum * nums[i];
            left.push(sum);
        }
        sum = 1;
        for i in (0..nums.len()).rev() {
            sum = sum * nums[i];
            right.push(sum);
        }
        right.reverse();
        let mut prod_array: Vec<i32> = Vec::new();
        
        for i in 0..nums.len() {
            if i == 0 {
                prod_array.push(right[i + 1]);
            } else if i == nums.len() - 1 {
                prod_array.push(left[i - 1]);
            } else {
                prod_array.push(left[i - 1] * right[i + 1]);
            }
        }
        prod_array
    }
}