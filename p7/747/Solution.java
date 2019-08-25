class Solution {
   	public int dominantIndex(int[] nums) {
		for (int i = 0; i < nums.length ;i++) {
			boolean flag = true;
			for (int k = 0; k < nums.length ; k++) {
				if (i != k && nums[i] < nums[k] << 1) {
					flag = false;
                    break;
				}
			}
			if (flag) return i;
		}
		return  -1;
	}
}