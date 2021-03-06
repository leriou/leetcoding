class Solution {
     public int trap(int[] height) {
        int n = height.length, ans = 0;
        if (n > 0) {
            int left = 0, right = n - 1;
            int l_max = height[0], r_max = height[n - 1];
            while (left <= right) {
                l_max = l_max > height[left] ? l_max : height[left];
                r_max = r_max > height[right] ? r_max : height[right]; 
                if (l_max < r_max) {
                    ans += l_max - height[left++];
                } else {
                    ans += r_max - height[right--];
                }
            }
        }
        return ans;
    }
}