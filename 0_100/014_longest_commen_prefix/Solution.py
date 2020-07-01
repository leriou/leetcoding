class Solution:
    def longestCommonPrefix(self, strs) -> str:
        length = len(strs)
        if length == 0:
            return ''
        if length == 1:
            return strs[0]
        loop = True
        pos = 0
        while loop:
            curr = None
            for i in strs:
                if len(i) == pos:
                    loop = False
                    pos -= 1
                    break
                if curr is None:
                    curr = i[pos]
                elif curr != i[pos]:
                    loop = False
                    pos -= 1
                    break
            pos+=1
        return strs[0][:pos]
