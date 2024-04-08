pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        counter.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    counter
        .iter()
        .max_by_key(|(key, value)| *value)
        .map(|(key, _)| *key)
        .unwrap_or(0)
}

/*
we don't need to know if one is the max or whatever
we need to know if there is a number that is the majority
there is an O(n) time and O(1) space complexity solution
Moore's voting algorithm
*/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut count = 0;
    for num in nums {
        if count == 0 {
            candidate = num;
        }
        // if its another of the same increment if not decrement
        if num == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}