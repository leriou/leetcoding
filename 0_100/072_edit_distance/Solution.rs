impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len()); 
        if m == 0 || n == 0 {
            return (m + n) as i32;
        }
        fn minix(a: i32, b: i32) -> i32 {
            if a > b {
                b
            } else {
                a        
            }
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0 as i32;n + 1];m + 1];
        for i in 1..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }

        for i in 1..= m {
            for j in 1..= n {
                let l = dp[i-1][j] + 1;
                let r = dp[i][j-1] + 1;
                let mut lr = dp[i-1][j-1];
                if word1.into_bytes()[i-1] != word2.into_bytes()[j-1] {
                    lr += 1;
                }
                dp[i][j] = minix(l, minix(r, lr));
            }
        }
        dp[m][n] 
    }
}