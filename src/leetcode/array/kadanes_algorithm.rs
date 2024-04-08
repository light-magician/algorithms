impl Solution {
    pub fn kadanes_algorithm(prices: Vec<i32>) -> i32 {
        /*
        this is one of those cheeky algorithms maybe kadaens algorithm
        track profit as zero
        track the lowest buys
        if you find a lower buy swap it
        */
        let mut profit = 0;
        let mut lowest_idx = 0 as usize;
        for i in 0..prices.len() {
            let current_sell = prices[i] - prices[lowest_idx];
            if current_sell > profit {
                profit = current_sell;
            }
            if prices[i] < prices[lowest_idx] {
                lowest_idx = i;
            }
        }
        profit
    }
}