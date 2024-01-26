
use std::collections::HashSet;

impl ContainsDuplicate {
    // i32 is a signed 32 bit int
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            // insert returns false if its already in there
            if !set.insert(num) {
                return true;
            }
        }
        false
    }
}