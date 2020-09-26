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
    public List<List<Integer>> pathSum(TreeNode root, int sum) {
        List<List<Integer>> ans = new ArrayList();
        List<Integer> cur = new ArrayList();
        dfs(root, sum, cur, ans);
        return ans;
    }
    
    public void dfs(TreeNode root, Integer target, List<Integer> curr, List<List<Integer>> ans) {
        if (root == null) return;
        curr.add(root.val);
        if (root.left == null && root.right == null && root.val == target) {
            ans.add(new ArrayList(curr));
        }
        if (root.left != null) dfs(root.left, target-root.val, curr, ans);
        if (root.right != null) dfs(root.right, target -root.val, curr, ans);
        curr.remove(curr.size() - 1);
    }
}