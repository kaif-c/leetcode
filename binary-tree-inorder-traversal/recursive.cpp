struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <vector>
using namespace std;

class Solution {
public:
    vector<int> inorderTraversal(TreeNode* root) {
        // Move from leftmost to rightmost

        if (!root)
            return {};

        if (!root->left && !root->right)
            return {root->val};

        vector<int> out;
        if (root->left)
            out = inorderTraversal(root->left);

        // We push regardless of anything else
        out.push_back(root->val);

        if (root->right) {
            auto all_right = inorderTraversal(root->right);
            out.insert(out.end(), all_right.begin(), all_right.end());
        }
        return out;
    }
};
