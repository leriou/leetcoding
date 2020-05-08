impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let (len, max, min) = (days.len(), days[days.len() - 1] as usize, days[0] as usize);
        let (mut dp, mut d,mut i) = (vec![0 as i32; max + 31], max, len - 1);
        while d >= min {
            if d == days[i] as usize {
                dp[d] = (dp[d + 1] + costs[0]).min(dp[d + 7] + costs[1]).min(dp[d + 30] + costs[2]);
                i -= 1;
            } else {
                dp[d] = dp[d + 1];
            }
            d -= 1;
        }
        dp[min]
    }
}
