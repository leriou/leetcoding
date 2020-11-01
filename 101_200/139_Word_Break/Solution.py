class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        n = len(s)
        dp = [True] + [False] * n
        for i in range(0, n):
            for j in range(i+1, n+1):
                if dp[i] and (s[i:j] in wordDict):
                    dp[j] = True
        return dp[-1]
