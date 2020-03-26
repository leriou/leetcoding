class Solution {
    public int numRookCaptures(char[][] board) {
        int size = board.length;
        int num = 0;
        for (int i = 0; i < size; i++) {
            for (int j = 0; j < size; j++) {
                if (board[i][j] == 'R') {
                    for (int p = i; p < size; p++) {
                        if (board[p][j] == 'p') {
                            num++;
                            p = size - 1;
                        }
                        if (board[p][j] == 'B') {
                            p = size - 1;
                        }
                    }
                    for (int q = i; q > 0; q--) {
                        if (board[q][j] == 'p') {
                            num++;
                            q = 0;
                        }
                        if (board[q][j] == 'B') {
                            q = 0;
                        }
                    }
                    for (int m = j; m < size; m++) {
                        if (board[i][m] == 'p') {
                            num++;
                            m = size - 1;
                        }
                        if (board[i][m] == 'B') {
                            m = size - 1;
                        }
                    }
                    for (int n = j; n > 0; n--) {
                        if (board[i][n] == 'p') {
                            num++;
                            n = 0;
                        }
                        if (board[i][n] == 'B') {
                            n = 0;
                        }
                    }
                }
            }
        }
        return num;
    }
}
