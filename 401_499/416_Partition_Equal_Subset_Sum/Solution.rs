impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().fold(0, |x1, x2| x1 + x2);
        if sum % 2 == 1 {
            return false;
        }
        sum >>= 1;
        let (n, mut dp) = (nums.len(), vec![false; sum as usize + 1]);
        dp[0] = true;
        for i in 0..n {
            for j in (0..=sum).rev() {
                if j >= nums[i] {
                    let ju = j as usize;
                    dp[ju] = dp[ju] || dp[ju - nums[i] as usize];
                };
            }
        }
        return dp[sum as usize];
    }
}
