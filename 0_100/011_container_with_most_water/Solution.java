class Solution {
    public int maxArea(int[] height) {
        if (height.length == 0) return 0;
        int left = 0, right = height.length - 1, area = 0;
        while (left < right) {
            int h, l = right - left;
            if (height[left] < height[right]) {
                h = height[left];
                ++left;
            } else {
                h = height[right];
                --right;
            }
            
            area = Math.max(area, l * h);
        }
        return area;
    }
}
