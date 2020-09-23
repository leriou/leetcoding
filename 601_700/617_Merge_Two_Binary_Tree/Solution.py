# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def mergeTrees(self, t1: TreeNode, t2: TreeNode) -> TreeNode:        
        def dfs(a, b):
            if a and b:
                a.val += b.val
                a.left = dfs(a.left, b.left)
                a.right = dfs(a.right, b.right)
            elif b:
                a = b
            return a    
        return dfs(t1, t2)
