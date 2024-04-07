impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        /*
        just do two pointers if there are more than one element
        get an iterator, while that iterator is in bounds
        check, 
        */
        if nums.len() < 1 {
            return nums.len().try_into().unwrap();
        }
        let mut lead: usize = 1;
        let mut tail: usize = 0;
        let mut end: usize = nums.len();
        while lead < end as usize {
            if nums[lead] == nums[tail] {
                nums.remove(lead);
                end -= 1;
            } else {
                lead += 1;
                tail += 1;
            }
        }
        return nums.len().try_into().unwrap();
    }
}