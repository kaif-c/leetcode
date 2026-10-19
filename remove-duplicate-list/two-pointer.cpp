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
    // Time = O(N)
    ListNode* deleteDuplicates(ListNode* head) {
        // Easier to just do a 2 pointer solution without a custom test
        // Simple 2 pointer
        // Read node and a write node
        // if next_node.val is cur_node.val then read_node = next_node
        // else both = next_node
        auto read_node = head;
        auto write_node = head;
        while (read_node != nullptr) {
            // If its a copy
            if (read_node->next && read_node->val == read_node->next->val) {
                read_node = read_node->next;
                continue;
            }
            write_node->val = read_node->val;
            read_node = read_node->next;
            if (read_node)
                write_node = write_node->next;
        }
        if (write_node)
            write_node->next = nullptr;
        return head;
    }
};
