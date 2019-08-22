class Solution {
   	public int dominantIndex(int[] nums) {
		for (int i = 0; i < nums.length ;i++) {
			int v = nums[i];
			boolean flag = true;
            int t = 0;
			for (int k = 0; k < nums.length ; k++ ) {
				t =  nums[k];
				if (i != k && v < t<<1 ) {
					flag = false;
                    break;
				}
			}
			if (flag) return i;
		}
		return  -1;
	}
}