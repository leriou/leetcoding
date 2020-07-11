class Solution {
    public int rob(int[] nums) {
        int length = nums.length;
        if (length == 0) return 0;
        if (length == 1) return nums[0];
        nums[1] = Math.max(nums[0],nums[1]);
        int tmp;
        for (int i = 2; i < length; ++i) {
            tmp = nums[1];
            nums[1] = Math.max(nums[0] + nums[i], nums[1]);
            nums[0] = tmp;
        }
        return nums[1];
    }
}