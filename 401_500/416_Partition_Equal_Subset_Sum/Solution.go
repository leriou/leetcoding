func canPartition(nums []int) bool {
    n := len(nums)
    sum := 0
    for i := 0; i < n; i++ {
        sum += nums[i]
    }
    if sum % 2 == 1 {
        return false
    } 
    sum >>= 1
    dp := make([]bool, sum + 1)
    dp[0] = true
    for i := 1; i < n; i++ {
        for j := sum; j >= 0; j-- {
            if j >= nums[i-1] {
                dp[j] = dp[j] || dp[j - nums[i-1]]
            }
        }
    } 
    return dp[sum]
}