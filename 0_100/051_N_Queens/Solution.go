import "strings"

var (
	ans   [][]string
	matrix [][]string
	row    int
)

func solveNQueens(n int) [][]string {
    ans    = make([][]string, 0)
    matrix = make([][]string, 0)
	row = n
	for i := 0; i < row; i++ {
		t := make([]string, row)
		for j := 0; j < row; j++ {
			t[j] = "."
		}
		matrix = append(matrix, t)
	}
	trail(0)
	return ans
}
func trail(i int) {
	if i >= row {
		t := make([]string, 0)
		for idx := 0; idx < row; idx++ {
			t = append(t, strings.Join(matrix[idx], ""))
		}
		ans = append(ans, t)
	} else {
		for j := 0; j < row; j++ {
			matrix[i][j] = "Q"
			if isValid(i, j) {
				trail(i + 1)
			}
			matrix[i][j] = "."
		}
	}
}
func abs(i int) int {
	if i > 0 {
		return i
	}
	return -i
}
func isValid(i, j int) bool {
	for m := 0; m < i; m++ {
		for p := 0; p < row; p++ {
			if j == p || abs(j-p) == abs(i-m) || (i+j) == (m+p) {
				if matrix[m][p] == "Q" {
					return false
				}
			}
		}
	}
	return true
}
