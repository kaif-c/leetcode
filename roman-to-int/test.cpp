#include "simple_loop_and_sub_or_add.cpp"
#include <vector>

vector<pair<string, int>> cases = {
    {"VII", 7  },
    {"IX",  9  },
    {"X",   10 },
    {"IV",  4  },
    {"III", 3  },
    {"CI",  101},
};

int main() {
    Solution s;
    for (auto cur_case: cases) {
        cout << "FOR " << cur_case.first << ": ";
        const int rom = s.romanToInt(cur_case.first);
        if (rom == cur_case.second)
            cout << "SUCCESS\n";
        else 
            cout << "FAIL. Got " << rom << " instead of " << cur_case.second << '\n';
    }
    return 0;
}
