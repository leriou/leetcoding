class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        a,b = [], []
        for i in S:
            if i == '#':
                if len(a) > 0: a.pop()
            else:
                a.append(i)
        for j in T:
            if j == '#':
                if len(b) > 0: b.pop()
            else:
                b.append(j)
        return "".join(a) == "".join(b)
