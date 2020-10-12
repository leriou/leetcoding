# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getMinimumDifference(self, root: TreeNode) -> int:
        m1 = [-1]
        pre = [-1]
        def dfs(node):
            if node:
                dfs(node.left)
                cur = node.val
                if m1[0] < 0:
                    if pre[0] < 0:
                        pass
                    else:
                        m1[0] = abs(cur-pre[0])
                else:
                    m1[0] = min(m1[0], abs(cur-pre[0]))
                pre[0] = cur
                dfs(node.right)
        dfs(root)
        return m1[0]
        
