class Solution:
    def getPermutation(self, n: int, k: int) -> str:
        arr = [i for i in range(1, n+1)]

        def helper(nums, k) -> str:
            totalNum = 1
            for i in range(1, len(nums)):
                totalNum *= i
            for i in range(0, len(nums)):
                sum_total = (i+1) * totalNum
                sub_total = k - sum_total
                if sub_total <= 0:
                    new_arr = []
                    new_arr += nums[:i]
                    new_arr += nums[i+1:]
                    return str(nums[i]) + helper(new_arr, k - (i * totalNum))
            return ""
        return helper(arr, k)
