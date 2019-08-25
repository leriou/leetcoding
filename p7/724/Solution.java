class Solution {
    public int pivotIndex(int[] nums) {
        Integer len = nums.length;
        for (int i = 0; i < len ;i++) {
            int leftSum = 0; 
            for (int j = 0;j<i;j++) {
                leftSum += nums[j];
            }  
            int rightSum = 0;
            for (int k = (i+1) ; k< len;k++) {
                rightSum += nums[k];
            }
            if (leftSum == rightSum) {
                return i;
            }
        }
        return -1;
    }
}