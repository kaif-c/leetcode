#include <iostream>
#include <variant>
#include <vector>
using namespace std;
#include "iterative.cpp"

class Case {
public:
    vector<variant<int, void*>> i;
    vector<int> o;
};

Case cases[] = {
    {{1,nullptr,2,3}, {1,3,2}},
    {{1,2,3,4,5,nullptr,8,nullptr,nullptr,6,7,9}, {4,2,6,5,7,1,3,9,8}},
    {{}, {}},
};

TreeNode* vec_to_tree(const vector<variant<int, void*>>& v) {
    if (v.empty() || !holds_alternative<int>(v[0]))
        return nullptr;

    vector<TreeNode*> nodes;
    nodes.reserve(v.size());

    for (const auto& x : v) {
        if (holds_alternative<int>(x))
            nodes.push_back(new TreeNode(get<int>(x)));
        else
            nodes.push_back(nullptr);
    }

    size_t child = 1;

    for (size_t i = 0; i < nodes.size() && child < nodes.size(); ++i) {
        if (!nodes[i]) continue;

        if (child < nodes.size())
            nodes[i]->left = nodes[child++];

        if (child < nodes.size())
            nodes[i]->right = nodes[child++];
    }

    return nodes[0];
}

string vec_to_str(const vector<int> &num) {
    string str = "[";
    for (size_t i = 0; i < num.size(); ++i) {
        str += to_string(num[i]);
        str += (i == num.size() - 1 ? "" : ", ");
    }
    return str + ']';
}

int main() {
    Solution s;
    for (auto &c: cases) {
        auto nodes = vec_to_tree(c.i);
        auto out = s.inorderTraversal(nodes);
        if (out != c.o)
            cerr << "FAIL: Got=" << vec_to_str(out) << " expected=" << vec_to_str(c.o) << endl;
        else
            cout << "SUCCESS" << endl;
    }
}
