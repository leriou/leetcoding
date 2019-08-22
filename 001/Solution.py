class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        maps = {}
        n = 0
        for i in nums:
            s = target - i 
            if maps.get(s) is not None:
                return [n, maps.get(s)]
            maps[i] = n
            n += 1
        
        