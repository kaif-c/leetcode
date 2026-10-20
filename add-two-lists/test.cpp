#include <iostream>
#include <string>
#include <vector>
using namespace std;
#include "in-place.cpp"

class Case {
public:
    vector<int> i1;
    vector<int> i2;
    vector<int> o;
    Case(vector<int> i1, vector<int> i2, vector<int> o):
        i1(i1), i2(i2), o(o) {}
};

Case cases[] = {
    {{9}, {9,9,9}, {8,0,0,1}},
    {{1}, {9,9}, {0,0,1}},
    {{2,4,9}, {5,6,4,9}, {7,0,4,0,1}},
    {{5,6},{5,4,9},{0,1,0,1}},
    {{2,4,3}, {5,6,4}, {7,0,8}},
    {{9,9,9,9,9,9,9}, {9,9,9,9}, {8,9,9,9,0,0,0,1}},
    {{}, {}, {}},
};

ListNode* vec_to_list(const vector<int>& v) {
    ListNode dummy;
    ListNode* tail = &dummy;

    for (int x : v) {
        tail->next = new ListNode(x);
        tail = tail->next;
    }

    return dummy.next;
}

string vec_to_str(vector<int> vec) {
    string v = "[";

    for (auto &cur: vec) {
        v += std::to_string(cur) + ", ";
    }

    return v + ']';
}

vector<int> list_to_vec(ListNode* head) {
    vector<int> v;

    while (head) {
        v.push_back(head->val);
        head = head->next;
    }

    return v;
}

int main() {
    Solution s;
    for (auto &c: cases) {
        auto out = list_to_vec(s.addTwoNumbers(vec_to_list(c.i1), vec_to_list(c.i2)));
        if (out != c.o)
            cerr << "FAIL: Got=" << vec_to_str(out) << " expected=" << vec_to_str(c.o) << endl;
        else
            cout << "SUCCESS" << endl;
    }
}
