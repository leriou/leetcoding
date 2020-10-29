# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumNumbers(self, root: TreeNode) -> int:  
        items = []
        nums = []
        def dfs(root, arr):
            if root is None: return root;
            arr.append(root.val)
            if root.left is None and root.right is None:
                nums.append(arr)
                return root
            if root.left is not None:
                dfs(root.left, arr.copy())
            if root.right is not None:
                dfs(root.right, arr.copy())       
        dfs(root, items)
        return sum([int("".join([str(n) for n in i])) for i in nums])
