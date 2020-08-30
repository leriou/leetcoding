class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        c_num, c = 0,1
        for i in nums:
            if i == c_num:
                c += 1
            else:
                c -= 1
                if c == 0:
                    c_num,c = i,1
        return c_num
