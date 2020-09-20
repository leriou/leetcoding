class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        ans = []
        n = len(nums)
        for i in range(0, 1 << n):
            t = []
            idx = 0
            for j in bin(i)[2:].zfill(n):
                if j == '1':
                    t.append(nums[idx])
                idx += 1
            ans.append(t)    
        return ans
