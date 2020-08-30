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
    int num = 0;
    TreeNode* convertBST(TreeNode* root) {
        if (root) {
            convertBST(root->right);
            num += root->val;
            root->val = num;
            convertBST(root->left);
        }
        return root;
    }
};
