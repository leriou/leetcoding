class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
       return len([i for i in bin(x^y) if i == '1'])
        
