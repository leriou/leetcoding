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
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> a = new ArrayList();
        h(0,root,a);
        return a;
    }
    
    void h(int level, TreeNode root, List<List<Integer>> a) {
        if (root == null) return;
        List<Integer> t = new ArrayList();
        if (a.size() > level) {
            t = a.get(level);
        }
        t.add(root.val);
        if (a.size() > level) {
            a.set(level, t);
        } else { 
            a.add(t);
        }
        h(level+1, root.left, a);
        h(level +1, root.right, a);
    }
}
