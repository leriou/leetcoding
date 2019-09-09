class Solution {
    public int trap(int[] height) {
        if (height.length == 0) return 0;
        int n = height.length;
        int left = 0, right = n - 1, ans = 0;
        int l_max = height[0];
        int r_max = height[n - 1];
        while (left <= right) {
            l_max = l_max > height[left] ? l_max : height[left];
            r_max = r_max > height[right] ? r_max : height[right];
            
            if (l_max < r_max) {
                ans += l_max - height[left];
                left++;
            } else {
                ans += r_max - height[right];
                right--;
            }
        }
        return ans;
        
    }
}
