impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().fold(0, |x1, x2| x1 + x2);
        if sum % 2 == 1 {
            return false;
        }
        sum /= 2;
        let (n, mut dp) = (nums.len(), vec![false; sum as usize + 1]);
        dp[0] = true;
        for i in 0..n {
            let mut j = sum;
            while j >= 0 {
                if j >= nums[i] {
                    dp[j as usize] = dp[j as usize] || dp[j as usize - nums[i] as usize];
                };
                j -= 1;
            }
        }
        return dp[sum as usize];
    }
}
