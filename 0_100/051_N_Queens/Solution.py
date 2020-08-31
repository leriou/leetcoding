class Solution:
    N = -1
    maze = []
    ans = []

    def _init_(self):
        for i in range(0, self.N):
            t = []
            for j in range(0, self.N):
                t.append(".")
            self.maze.append(t)

    def solveNQueens(self, n: int) -> List[List[str]]:
        self.N = n
        self._init_()
        self.trail(0)
        return self.ans

    def trail(self, n):
        if n >= self.N:
            self.ans.append(["".join(i) for i in self.maze])
        else:
            for i in range(0, self.N):
                self.maze[n][i] = "Q"
                if self.is_valid(n, i):
                    self.trail(n+1)
                self.maze[n][i] = "."

    def is_valid(self, x, y):
        for i in range(0, x):
            for j in range(0, self.N):
                if (y == j) or (abs(x-i) == abs(y-j)) or (x+y) == (i+j):
                    if self.maze[i][j] == "Q":
                        return False
        return True
