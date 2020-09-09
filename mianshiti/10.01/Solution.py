class Solution:
    def merge(self, A: List[int], m: int, B: List[int], n: int) -> None:
        """
        Do not return anything, modify A in-place instead.
        """
        p = len(A)
        while n > 0 and m > 0:
            p-=1
            if A[m-1] > B[n-1]:
                A[p] = A[m-1]
                m-= 1
            else:
                A[p] = B[n-1] 
                n -= 1
        while m > 0 and p > 0:
            A[p-1] = A[m-1]
            m -= 1
            p -= 1
        while n > 0 and p > 0:
            A[p-1] = B[n-1]
            n -= 1
            p -= 1
