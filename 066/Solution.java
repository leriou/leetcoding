class Solution {
    public int[] plusOne(int[] nums) {
        int carry = 1;
        int idx = nums.length - 1;
        while (idx >= 0) {
            if (carry == 1) {
                int tail = nums[idx] + carry;
                if (tail >= 10) {
                    tail -= 10;  
                } else {
                    carry = 0;
                }
                nums[idx] = tail;
            }
            idx--;
        }
        // 进位数字未消耗完毕
        if (carry == 1) {
            int[] c = new int[nums.length + carry];
            c[0] = carry;
            for (int i = 0; i < nums.length; i++) {
                c[i + carry] = nums[i];
            }
            return c;
        }
        return nums;
    }
}