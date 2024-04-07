impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        /*
        we are going to read it backwards
            this lets us place in correct order in one go
        nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
        we can do this when i and j >= 0
        can do two whiles at the end to get what remains
        */
        // have to convert all i32 values to usize to use as iterators
        // end of values at nums1
        let mut i: i32 = m - 1;
        // end of values at nums2
        let mut j: i32 = n  - 1;
        // end of array at nums1, which has tail zeros
        let mut end: i32 = (m + n) - 1;
        
        while i >= 0 && j >= 0 {
            // if else either nums1 is greater or not
            if nums1[i as usize] > nums2[j as usize] {
                nums1[end as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[end as usize] = nums2[j as usize];
                j -= 1;
            }
            end -= 1;
        }
        /*
        at the end there will be some left over
        basically we can count backwards from i or j
        */
        while i >= 0 {
            nums1[end as usize] = nums1[i as usize];
            i -= 1;
            end -= 1;
        }
        while j >= 0 {
            nums1[end as usize] = nums2[j as usize];
            j -= 1;
            end -= 1;
        }
    }
}