struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (!root)
            return false;
        // Simply recurse through using DFS
        // at each step adding to the root
        return curPathHasSum(root, 0, targetSum);
    }
    bool curPathHasSum(TreeNode *root, int curVal, int targetSum) {
        // Left one found sum, so terminate immediately
        if (root->left && curPathHasSum(root->left, root->val + curVal, targetSum)) {
            return true;
        }
        // Right one found sum, so terminate immediately
        if (root->right && curPathHasSum(root->right, root->val + curVal, targetSum)) {
            return true;
        }
        // We are at the end (leaf) node
        if (!root->left && !root->right)
            return root->val + curVal == targetSum;
        return false;
    }
};
