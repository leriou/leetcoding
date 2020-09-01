impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = vec![];
        let row = n as usize;
        let mut board: Vec<Vec<&str>> = vec![vec!["."; n as usize]; n as usize];
        Solution::backtrace(0, row, &mut ans, &mut board);
        ans
    }

    pub fn backtrace(n: usize, row: usize, ans: &mut Vec<Vec<String>>, board: &mut Vec<Vec<&str>>) {
        if n >= row {
            ans.push(board.iter().map(|x| x.join("").to_string()).collect());
        } else {
            for i in 0..row {
                board[n][i] = "Q";
                if Solution::is_valid(board, row, n, i) {
                    Solution::backtrace(n + 1, row, ans, board);
                }
                board[n][i] = ".";
            }
        }
    }

    pub fn is_valid(board: &Vec<Vec<&str>>, row: usize, x: usize, y: usize) -> bool {
        for i in 0..x {
            for j in 0..row {
                let f = (x as i32 - i as i32).abs() == (y as i32 - j as i32).abs();
                if y == j || (x + y) == (i + j) || f {
                    if board[i][j] == "Q" {
                        return false;
                    }
                }
            }
        }
        true
    }
}