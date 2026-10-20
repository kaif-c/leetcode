struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    // Space = O(1)
    // Time = O(max(n, m))
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        // Some type of in place  modification
        if (!l1 && !l2)
            return nullptr;
        int carry = 0;
        auto cur = l1;
        auto oth = l2;
        ListNode *prev = nullptr;

        const auto head = cur;
        while (l1 || l2 || carry) {
            int val = 0;
            // Get values and move readers
            if (l1) {
                val += l1->val;
                l1 = l1->next;
            }
            if (l2) {
                val += l2->val;
                l2 = l2->next;
            }
            // Add
            val += carry;
            carry = 0;
            if (val > 9) {
                val -= 10;
                carry = 1;
            }
            cur->val = val;

            // Move writers
            auto tmp = cur->next;
            if (oth) {
                cur->next = oth;
                oth = tmp;
            }
            prev = cur;
            cur = cur->next;
        }
        prev->next = nullptr;
        return head;
    }
};
