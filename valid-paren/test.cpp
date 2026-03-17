#include <iostream>
#include <utility>
#include <vector>
#include "simple-forward-loop.cpp"

static const vector<pair<string, bool>> cases = {
    {"[[[()]]]", true},
    {"[)](",     false},
    {"(())]",    false},
    {"({))]",    false},
    {"{([[]])}", true},
};

int main() {
    Solution s;
    for (auto cur_case: cases) {
        if (s.isValid(cur_case.first) == cur_case.second) 
            cout << "SUCCESS\n";
        else 
            cout << "ERROR WITH '" << cur_case.first << "', got " <<cur_case.second << '\n';
    }
}
