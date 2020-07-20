class Solution {
    public boolean canPartition(int[] nums) {
        int len = nums.length, sum = 0;
        for (int i: nums) sum += i;
        if (sum % 2 == 1) return false;
        sum >>= 1;
        boolean[] dp = new boolean[sum + 1];
        dp[0] = true;
        for (int i = 0; i < len; ++i) {
            for (int j = sum; j >= 0; --j) {
                if (j >= nums[i]) 
                    dp[j] = dp[j] || dp[j - nums[i]];
            }
        }
        return dp[sum];
    }
}
