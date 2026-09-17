#include "simple-check.cpp"
#include <iostream>

class Case {
public:
    vector<string> list;
    string prefix;
};

static Case cases[] = {
    {{"flower","flow","flight"}, "fl"},
    {{"dog","racecar","car"}, ""},
    {{"sussy", "sussy", "sussy"}, "sussy"},
};

int main() {
    Solution s;
    for (auto &cur_case: cases) {
        const auto val = s.longestCommonPrefix(cur_case.list);
        if (val != cur_case.prefix)
            cerr << "ERROR: Expected '" << cur_case.prefix << "' but got '"
                << val << "' instead\n";
        else
            cout << "SUCCESS\n";
    }
}
