class Solution {
    public void moveZeroes(int[] nums) {
        int pos = 0;
        for (int i = 0; i < nums.length ; i++) {
            if (nums[i] != 0) {
                int tmp = nums[i];
                nums[i] = nums[pos];
                nums[pos++] = tmp;
            }
        }
    }
}