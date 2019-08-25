class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        pos = 0 
        i = 0
        for n in nums:
            if n != 0:
                nums[pos],nums[i] = nums[i], nums[pos] 
                pos = pos + 1
            i = i + 1