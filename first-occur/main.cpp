#include <iostream>
#include "match-char.cpp"

class Case {
public:
    string needle;
    string haystack;
    int pos;
};

Case cases[] = {
    {"sad", "sadbusted", 0},
    {"sad", "sadbutsad", 0},
    {"leetcodw", "leetcode", -1},
    {"ll", "hello", 2}
};

int main() {
    Solution s;
    for (auto &c: cases) {
        if (s.strStr(c.haystack, c.needle) != c.pos)
            cout << "FAILED: " << c.haystack << '\n';
        else
            cout << "SUCCES\n";
    }
}
