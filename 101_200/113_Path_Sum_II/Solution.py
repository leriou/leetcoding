# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
        ans = []
        t = []

        def dfs(n, target, cur):
            if not n:
                return
            cur.append(n.val)
            if not n.left and not n.right and target == n.val:
                ans.append(cur.copy())
            if n.left:
                dfs(n.left, target - n.val, cur)
            if n.right:
                dfs(n.right, target - n.val, cur)
            cur.pop()
        dfs(root, sum, t)
        return ans
