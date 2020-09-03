from functools import lru_cache


class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        self.memo = {}
        return self.dp(s, 0, p, 0)

    @lru_cache(maxsize=None)
    def dp(self, s, i, p, j):
        if j == len(p):
            return i == len(s)
        if i == len(s):
            if (len(p) - j) % 2 == 1:
                return False
            while j+1 < len(p):
                if p[j+1] != '*':
                    return False
                j += 2
            return True
        res = False
        if s[i] == p[j] or p[j] == '.':
            if j < len(p) - 1 and p[j+1] == '*':
                res = self.dp(s, i+1, p, j) or self.dp(s, i, p, j+2)
            else:
                res = self.dp(s, i+1, p, j+1)
        else:
            if j < len(p) - 1 and p[j+1] == '*':
                res = self.dp(s, i, p, j+2)
            else:
                res = False
        return res
