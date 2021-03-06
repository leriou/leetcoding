# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []
        ans = []
        arr = []
        self.backtrace(root, arr, ans)
        return ans
    
    def backtrace(self, root, arr, ans):
        arr.append(str(root.val))
        if not root.left and not root.right:
            ans.append("->".join(arr))
        else:
            if root.left:
                self.backtrace(root.left, arr, ans)
                arr.pop()
            if root.right:
                self.backtrace(root.right, arr, ans)
                arr.pop()
