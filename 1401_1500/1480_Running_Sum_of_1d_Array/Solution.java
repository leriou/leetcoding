class Solution {
    public int[] runningSum(int[] nums) {
        int length = nums.length;
        if (length == 0) return nums;
        for (int i = 1;i< length; ++i) {
            nums[i] = nums[i]+ nums[i-1];
        }
        return nums;
    }
}
