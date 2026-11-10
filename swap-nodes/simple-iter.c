struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode* swapPairs(struct ListNode* head) {
    // Edge case
    if (!head || !head->next)
        return head;

    // Ret as the first 2 swapped
    register struct ListNode *const ret = head->next;
    // The current iterator, delta = +2
    register struct ListNode *cur = head;
    // Hold the previous element to make sure its next changes
    register struct ListNode *prev = 0;

    while (cur && cur->next) {
        if (prev)
            prev->next = cur->next;
        prev = cur;
        // Temporary value storing cur + 2
        register struct ListNode *const next_cur = cur->next->next;
        // Swap
        cur->next->next = cur;
        cur->next = next_cur;
        cur = next_cur;
    }
    return ret;
}
