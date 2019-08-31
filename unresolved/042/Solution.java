class Solution {
    public int trap(int[] height) {
        Stack<Integer> stk = new Stack<>();
        int n = height.length;
        if (n == 0) {
            return 0;
        }
        int res = 0;
        for (int i = 0; i < n; i++) {
            while (!stk.isEmpty() && height[stk.peek()] <= height[i]) {
                int hh = stk.pop();
                if (!stk.isEmpty()) {
                    int h = Math.min(height[stk.peek()], height[i]) - height[hh];
                    int w = i - stk.peek() - 1;
                    res += h * w;
                }
            }
            stk.push(i);
        }
        return res;
    }
}