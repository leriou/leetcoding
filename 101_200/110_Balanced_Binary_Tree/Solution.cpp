/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    bool isBalanced(TreeNode* root) {
        if (!root) {
            return true;
        } else {
            return isBalanced(root->left) 
                && isBalanced(root->right)
                && abs(helper(root->left) - helper(root->right)) <= 1;
        }
    }
    
    int helper(TreeNode* root) {
        if (!root) {
            return 0; 
        } else {
            return max(helper(root->left),helper(root->right)) + 1;
        }
    }
};