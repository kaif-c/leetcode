struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        // It does not ask for an in place modifications
        // So i will pick a simple new list creation
        if (!l1 && !l2)
            return nullptr;
        auto head = new ListNode(0);
        auto cur = head;
        int carry = 0;
        while (l1 || l2 || carry) {
            if (l1) {
                cur->val += l1->val;
                l1 = l1->next;
            }
            if (l2) {
                cur->val += l2->val;
                l2 = l2->next;
            }

            cur->val += carry;
            carry = 0;
            if (cur->val > 9) {
                cur->val -= 10;
                carry = 1;
            }

            if (l1 || l2 || carry)
                cur->next = new ListNode(0);
            else
                cur->next = nullptr;

            cur = cur->next;
        }
        return head;
    }
};
