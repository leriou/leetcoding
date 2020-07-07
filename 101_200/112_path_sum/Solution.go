/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func hasPathSum(root *TreeNode, sum int) bool {
  if root == nil {
    return false;
  }
  if root.Left == nil && root.Right == nil {
    return sum == root.Val;
  }
  return (root.Left != nil && hasPathSum(root.Left, sum - root.Val)) || (root.Right!=nil&&hasPathSum(root.Right, sum-root.Val));
}
