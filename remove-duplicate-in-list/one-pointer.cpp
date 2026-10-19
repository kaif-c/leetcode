struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    // Space = O(1) (64-bytes only)
    // Time = O(N)
    ListNode* deleteDuplicates(ListNode* head) {
        // Simpler 1 poinnter approach
        // Set cur->next to the first element that isn't the same
        auto cur = head;
        while (cur && cur->next) {
            if (cur->val == cur->next->val) {
                cur->next = cur->next->next;
            }
            else {
                cur = cur->next;
            }
        }
        return head;
    }
};
