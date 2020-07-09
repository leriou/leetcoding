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
    public TreeNode invertTree(TreeNode root) {
        if (root == null) return root;
        TreeNode tmp = root.right;
        root.right = root.left;
        root.left = tmp;
        this.invertTree(root.left);
        this.invertTree(root.right);
        return root;
    }
}
