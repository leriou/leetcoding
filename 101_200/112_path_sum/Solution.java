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
    public boolean hasPathSum(TreeNode root, int sum) {
      if (root == null) {return false;}
      
       if (root.left == null && root.right == null) {
        return sum == root.val;
      }
      return (root.left != null && this.hasPathSum(root.left, sum- root.val)) || (root.right != null && this.hasPathSum(root.right,sum-root.val));
    }
}