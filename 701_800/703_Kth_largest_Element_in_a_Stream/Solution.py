class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        self.v = [None]
        self.k = k
        for i in nums:
            self.add(i)

    def compare(self, a, b):
        return self.v[a] < self.v[b]

    def heapify(self, n):
        last = len(self.v) - 1
        while 2*n <= last:
            j = 2*n
            if j < last and self.compare(j+1, j):
                j += 1
            if self.compare(n, j):
                break
            self.v[n], self.v[j] = self.v[j], self.v[n]
            n = j

    def add(self, val: int) -> int:
        size = len(self.v) - 1
        if size < self.k:
            self.v.append(val)
            last = len(self.v) - 1
            parent = last >> 1
            while self.v[parent] is not None and self.v[parent] > self.v[last]:
                self.v[parent], self.v[last] = self.v[last], self.v[parent]
                last = parent
                parent = last >> 1
        else:
            if self.v[1] < val:
                self.v[1] = val
                self.heapify(1)
        ans = self.v[1]
        return ans
