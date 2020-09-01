class Solution {
    
    private int row = 0;
    List<List<String>> ans = new ArrayList();
    String[][] board;
    public List<List<String>> solveNQueens(int n) {
        board = new String[n][n];
        for (String[] s: board) 
            Arrays.fill(s,".");
        this.row = n;
        trail(0);
        return ans;
    }
    
    void trail(int n) {
        if (n == this.row) {
            List<String> t = new ArrayList();
            for (String[] i : board) {
                t.add(String.join("", Arrays.asList(i)));
            }
            ans.add(t);
        } else {
            for (int i = 0; i < row; i++) {
                board[n][i] = "Q";
                if (valid(n,i)) {
                    trail(n+1);
                }
                board[n][i] = "."; 
            }
        }
    }
    boolean valid(int x,int y) {
        for (int i = 0; i < x; i++) {
            for (int j = 0; j < row; j++) {
                if (y == j || Math.abs(x-i) == Math.abs(y-j) || (x+y) == (i+j)) {
                    if ("Q".equals(board[i][j]))
                        return false;
                }
            }
        }
        return true;
    }
}
