impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        /*
        two pointers
        if a value is smaller, move that pointer inward
        track max
        */
        let mut max: i32 = 0;
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        while l < r {
            let check = Self::store(l, r, height[l], height[r]);
            if check > max {
                max = check
            }
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        // once again this is not python, you have to ensure something is returned
        return max;
    }

    pub fn store(i1: usize, i2: usize, h1: i32, h2: i32) -> i32 {
        // min of heights times differences between indexes as i32
        let distance = (i2 - i1) as i32;
        // in rust ints are like tensors pytorch, min is a function of that int
        return h1.min(h2) * distance;
    }
}