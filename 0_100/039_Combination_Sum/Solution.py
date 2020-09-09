class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        self.ans = []
        self.arr = []
        self.c = candidates
        self.dfs(0, 0, target)
        return self.ans
        
    def dfs(self, o, cur, t):
        if cur == t:
            self.ans.append(self.arr.copy())
        elif cur > t:
            return
        else:
            while o < len(self.c):
                self.arr.append(self.c[o])
                cur += self.c[o]
                self.dfs(o, cur, t)  
                cur -= self.c[o]
                self.arr.pop()
                o += 1
