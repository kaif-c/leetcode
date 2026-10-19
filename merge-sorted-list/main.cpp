#include "./forward-loop.cpp"
#include <cstdlib>
#include <iostream>
#include <string>
#include <vector>
#include <memory.h>
#include <cstdio>
using namespace std;

ListNode *list_from_vector(vector<int> v) {
    fflush(stderr);
    ListNode *holder = (ListNode*)malloc(sizeof(ListNode) * v.size());
    for (int i = 0; i < v.size(); ++i) {
        holder[i].val = v[i];
        holder[i].next = holder + i + 1;
    }
    holder[v.size() - 1].next = nullptr;
    return holder;
}

class Case {
public:
    ListNode *l1;
    ListNode *l2;
    ListNode *out;
};

string list_to_string(ListNode *l) {
    string str = "[";
    while (l) {
        str += to_string(l->val);
        l = l->next;
        if (l)
            str += ',';
    }
    return str + ']';
}

bool try_case(Solution &s, Case &c) {
    const auto got = s.mergeTwoLists(c.l1, c.l2);
    cout << "Got: " << list_to_string(got) << '\n';
    for (auto g = got, expected = c.out; g && c.out; (g = g->next), (c.out = c.out->next)) {
        if ((c.out && !g) || (g && !c.out))
            return false;
        if (g->val != c.out->val) {
            cerr << "[ERROR]: Expected " << c.out->val << " but got " << g->val << '\n';
            return false;
        }
        if ((c.out && !g) || (g && !c.out))
            return false;
    }
    return true;
}

int main() {
    Solution s;
    Case cases[] = {
        { .l1 = list_from_vector({1, 3, 7}), .l2 = list_from_vector({1, 2, 5}), .out = list_from_vector({1, 1, 2, 3, 5, 7}) },
        { .l1 = list_from_vector({2, 4, 6}), .l2 = list_from_vector({1, 3, 5}), .out = list_from_vector({1, 2, 3, 4, 5, 6}) },
        { .l1 = list_from_vector({}), .l2 = list_from_vector({}), .out = list_from_vector({}) },
        { .l1 = list_from_vector({}), .l2 = list_from_vector({0}), .out = list_from_vector({0}) },
        { .l1 = list_from_vector({5, 6, 7}), .l2 = list_from_vector({1}), .out = list_from_vector({1, 5, 6, 7}) },
    };
    for (auto c: cases) {
        if (!try_case(s, c))
            cerr << "EXPECTED " << list_to_string(c.out) << '\n';
        else
            cout << "SUCCESS\n";
    }
}
