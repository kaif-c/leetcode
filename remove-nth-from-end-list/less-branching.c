struct ListNode {
    int val;
    struct ListNode *next;
} ListNode;

// Time = O(n)
// Space = O(1)
struct ListNode* removeNthFromEnd(struct ListNode* head, int n) {
    // n <= len(head)
    // sz <= 30

    // As we move we first reach the nth node
    // Then we set to_swap to head
    // The next iteration we set before_to_swap
    // Now we keep iterating through both head, to_swap, and before_to_swap
    // until we reach the end
    // Now to_swap & before_to_swap is set correctly
    unsigned char offset = 0;
    struct ListNode *before_to_rem = 0;
    struct ListNode *iter = head;

    // Find nth from begining
    while (iter) {
        ++offset;
        iter = iter->next;
        if (offset > n) {
            before_to_rem = head;
            break;
        }
    }

    // Iterate till end to get nth from end
    // Seperate this iteration to reduce branching
    while (iter) {
        before_to_rem = before_to_rem->next;
        iter = iter->next;
    }

    // Actually remove

    // Remove the first element
    if (!before_to_rem)
        return head->next;

    // Remove from middle
    if (before_to_rem->next)
        before_to_rem->next = before_to_rem->next->next;
    return head;
}
