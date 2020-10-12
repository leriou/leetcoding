impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        let mut dp = vec![1_i32; n as usize];
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        for next in 1..n as usize {
            dp[next] = (dp[p2] * 2).min(dp[p3] * 3).min(dp[p5] * 5);
            if dp[next] == dp[p2] * 2 {
                p2 += 1;
            }
            if dp[next] == dp[p3] * 3 {
                p3 += 1;
            }
            if dp[next] == dp[p5] * 5 {
                p5 += 1;
            }
        }
        dp[n as usize - 1]
    }
}
