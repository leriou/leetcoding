impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; b.len()]; a.len()];
        let mut max = 0;
        for i in 0..a.len() {
            for j in 0..b.len() {
                if a[i] == b[j] {
                    if i > 0 && j > 0 {
                        dp[i][j] = dp[i - 1][j - 1] + 1;
                    } else {
                        dp[i][j] = 1;
                    };
                    max = max.max(dp[i][j]);
                }
            }
        }
        max
    }
}
