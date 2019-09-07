class Solution {
	public int dominantIndex(int[] nums) {
	 int max = -1; int sec = -1; int idx = 0;
	 for (int i = 0; i < nums.length ;i++) {
		 if (nums[i] > max) {
			 sec = max;
			 max = nums[i];
			 idx = i;
		 } else if (nums[i] > sec) {
			 sec = nums[i];
		 }
	 }
	 return max - sec >= sec ? idx : -1;
 }
}