class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
       return [i for i in bin(x^y)][2:].count('1')
        
