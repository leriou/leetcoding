class Solution:
    
    def permute(self, nums: List[int]) -> List[List[int]]:
        self.ans = []
        arr = []
        self.backtrace(nums, arr)
        return self.ans
    
    def backtrace(self, nums, arr):
        if len(nums) == len(arr):
            self.ans.append([i for i in arr])
        else:
            for i in nums:
                if i not in arr:
                    arr.append(i)
                    self.backtrace(nums, arr)
                    arr.pop()
