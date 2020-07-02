class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        l,i,m,rm = len(prices), len(prices)-1, 0,0
        if l > 1:
            while i >= 0:
                m = max(0, max(m, rm - prices[i]))
                rm = max(rm, prices[i])
                i-=1
        return m
