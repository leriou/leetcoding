import (
	"strings"
)

func solveNQueens(n int) [][]string {
	ans := make([][]string, 1)
	matrix := make([][]int, n+1)
	for i := 0; i < n+1; i++ {
		matrix[i] = make([]int, n+1)
	}
	return trail(1, n, matrix, ans)[1:]
}
func trail(i, n int, matrix [][]int, ans [][]string) [][]string {
	if i > n {
		ans = addMatrix(n, matrix, ans)
	} else {
		for j := 1; j <= n; j++ {
			matrix[i][j] = 1
			if isValid(matrix, n, i, j) {
				ans = trail(i+1, n, matrix, ans)
			}
			matrix[i][j] = 0
		}
	}
	return ans
}
func abs(i int) int {
	if i > 0 {
		return i
	}
	return -i
}
func addMatrix(n int, matrix [][]int, ans [][]string) [][]string {
	line := make([]string, 1)
	for i := 1; i < n+1; i++ {
		s := ""
		for j := 1; j < n+1; j++ {
			if matrix[i][j] == 1 {
				s += "Q"
			} else {
				s += "."
			}
		}
		line = append(line, strings.TrimSpace(s))
	}
	ans = append(ans, line[1:])
	return ans
}
func isValid(matrix [][]int, n, i, j int) bool {
	for m := 1; m < i; m++ {
		for p := 1; p <= n; p++ {
			if matrix[m][p] == 1 {
				if j == p || abs(j-p) == abs(i-m) {
					return false
				}
			}
		}
	}
	return true
}
