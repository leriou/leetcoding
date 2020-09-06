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
    public List<List<Integer>> levelOrderBottom(TreeNode root) {
        List<List<Integer>> ans = new ArrayList();
        backtrace(root,ans, 0);
        Collections.reverse(ans);
        return ans;
    }
    
    void backtrace(TreeNode root, List<List<Integer>> ans, int level) {
        if (root == null) return ;
        if (ans.size() < level+1) {
            ans.add(new ArrayList());
        }
        List<Integer> arr = ans.get(level);
        arr.add(root.val);
        backtrace(root.left, ans, level+1);
        backtrace(root.right, ans,level+1);
    }
}