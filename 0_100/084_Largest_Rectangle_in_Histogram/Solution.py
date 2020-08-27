class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        heights.insert(0,0)
        heights.append(0)
        n, ans, st = len(heights), 0, []
        for i in range(0,n):
            while len(st) != 0 and heights[i] < heights[st[len(st) - 1]]:
                h = heights[st.pop()]
                ans = max(ans, (i - st[len(st) - 1] -1) * h)
            st.append(i)
        return ans