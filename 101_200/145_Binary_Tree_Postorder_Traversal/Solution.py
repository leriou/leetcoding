# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        ans, st = [], []
        if not root: return ans
        st.append(root)
        while len(st) > 0:
            t = st.pop()
            if t:
                st.append(t)
                st.append(None)
                if t.right: st.append(t.right)
                if t.left: st.append(t.left)
            else:
                ans.append(st.pop().val)
        return ans
