class Solution {
    public int maxProfit(int[] prices) {
        int l = prices.length;
        int i = prices.length-1;
        int m = 0;
        int rm = 0;
        if (l > 1) {
            while (i >= 0) {
                m = Math.max(0, Math.max(m, rm - prices[i]));
                rm = Math.max(rm, prices[i]);
                i--;
            }
        }
        return m;
    }
}