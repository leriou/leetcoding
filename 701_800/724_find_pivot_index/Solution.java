class Solution {
    public int pivotIndex(int[] nums) {
        int len = nums.length;
        int total = 0;
        for (int i = 0; i < len; i++) {
           total += nums[i];
        }
        int leftSum = 0;
        for (int j = 0; j< len; j++) {
            if (total - nums[j] - leftSum == leftSum) return j;
            leftSum += nums[j];
        }
        return -1;
    }
}