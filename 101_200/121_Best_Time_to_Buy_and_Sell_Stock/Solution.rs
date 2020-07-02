impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0i32;
        for i in 0..prices.len() {
            let mut left_max = 0i32;
            let pos = i.min(prices.len());
            for j in pos..prices.len() {
                let delta = prices[j] - prices[i];
                left_max = delta.max(0).max(left_max);
            }
            max = left_max.max(max);
        }
        max
    }
}
