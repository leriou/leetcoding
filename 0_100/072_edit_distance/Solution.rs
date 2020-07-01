impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        if m == 0 || n == 0 {
            return (m + n) as i32;
        }
        let (wb1, wb2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=n {
                dp[i][j] = if i == 0 {
                    j
                } else if j == 0 {
                    i
                } else {
                    let l = dp[i - 1][j] + 1;
                    let r = dp[i][j - 1] + 1;
                    let mut lr = dp[i - 1][j - 1];
                    if wb1[i - 1] != wb2[j - 1] {
                        lr += 1;
                    }
                    l.min(r.min(lr))
                }
            }
        }
        dp[m][n] as i32
    }
}
