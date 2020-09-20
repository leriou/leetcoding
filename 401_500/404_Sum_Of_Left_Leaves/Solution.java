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
    public int sumOfLeftLeaves(TreeNode root) {
        return fs(root, false);
    }
    
    private int fs(TreeNode n, boolean b) {
        if (n == null) return 0;
        if (n.left == null && n.right == null) return b ? n.val : 0;
        return fs(n.left, true) + fs(n.right, false);
    }
}
