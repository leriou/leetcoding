class Solution {
    public int minDistance(String word1, String word2) {
        int i = 0, j = 0, m = word1.length(), n = word2.length();
        if (m == 0 || n == 0) return m + n;
        int[][] d = new int[m + 1][n + 1];
        for (; i < m + 1; i++) {
            d[i][0] = i; 
        }
        for (; j < n + 1; j++) {
            d[0][j] = j;
        }
        for (i = 1; i < m + 1; i++) {
            for (j = 1; j < n + 1; j++) {
                int l = d[i-1][j] + 1;
                int r = d[i][j-1] + 1;
                int lr = d[i-1][j-1];
                if (word1.charAt(i - 1) != word2.charAt(j - 1)) {
                    lr += 1;
                }
                d[i][j] = Math.min(l, Math.min(r, lr));   
            }
        }
        return d[m][n];
    }
}
