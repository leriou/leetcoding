from collections import Counter

class Solution:
    def commonChars(self, A: List[str]) -> List[str]:
        res = None
        for a in A:
            c = Counter(a)
            if res is None:
                res = c
            else:
                res &= c
        return list(res.elements())
