class Solution:
    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        row, col = len(matrix),len(matrix[0])
        left, right = matrix[0][0], matrix[row-1][col-1] 
        while left < right:
            mid = (left + right) >> 1
            if self.lessThanMid(matrix, mid) < k:
                left = mid + 1
            else:
                right = mid
        return left
    
    def lessThanMid(self, matrix, mid):
        r, c = len(matrix), len(matrix[0]) 
        i, j, count = r - 1, 0, 0
        while i >= 0 and j < c:
            if matrix[i][j] <= mid:
                count += i + 1
                j += 1
            else:
                i -= 1
        return count
