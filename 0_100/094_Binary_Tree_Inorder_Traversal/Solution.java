/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
    public List<Integer> inorderTraversal(TreeNode root) {
        List<Integer> a = new ArrayList();
        s(a,root);
        return a;
    }
    
    public void s(List<Integer> a, TreeNode root) {
        if (root == null) return;
        if (root.left != null) s(a,root.left);
        a.add(root.val);
        if (root.right != null) s(a,root.right);
    }
}
