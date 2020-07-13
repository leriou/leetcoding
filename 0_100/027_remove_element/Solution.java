class Solution {
    public int removeElement(int[] nums, int val) {
        int a = 0;
        for (int j = 0; j < nums.length; ++j) {
            if (nums[j] != val) {
                nums[a] = nums[j];
                ++a;
            }
        }
        return a;
    }
}
