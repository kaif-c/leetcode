 struct ListNode {
     int val;
     ListNode *next;
     ListNode() : val(0), next(nullptr) {}
     ListNode(int x) : val(x), next(nullptr) {}
     ListNode(int x, ListNode *next) : val(x), next(next) {}
 };

class Solution {
public:
    /// Space: O(1), Time: O(N*m) where N = list1.size(), m = list2.size()
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        if (!list1)
            return list2;
        if (!list2)
            return list1;

        /// Manually compute the first values
        ListNode *it = list1;
        ListNode *other_it = list2;
        if (list1->val > list2->val) {
            it = list2;
            other_it = list1;
        }
        auto head = it;

        /// Loop over the rest
        while (true) {
            /// When we reach the end of either lists
            /// Makes sure we exit before we compute any nullptr
            if (!other_it)
                return head;
            if (!it->next) {
                it->next = other_it;
                return head;
            }

            /// Comparison
            if (it->next->val > other_it->val) {
                auto temp = it->next;
                it->next = other_it;
                other_it = temp;
            }
            it = it->next;
        }
    }
};
