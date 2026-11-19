#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include "less-branching.c"

// Test utilities
struct ListNode* createList(const int *arr, int size) {
    struct ListNode *head = NULL;
    struct ListNode **tail = &head;

    for (int i = 0; i < size; i++) {
        *tail = malloc(sizeof(struct ListNode));
        (*tail)->val = arr[i];
        (*tail)->next = NULL;
        tail = &(*tail)->next;
    }

    return head;
}

void freeList(struct ListNode *head) {
    while (head) {
        struct ListNode *next = head->next;
        free(head);
        head = next;
    }
}

bool compareList(struct ListNode *head, const int *expected, int size) {
    for (int i = 0; i < size; i++) {
        if (!head || head->val != expected[i])
            return false;

        head = head->next;
    }

    return head == NULL;
}

void printList(struct ListNode *head) {
    printf("[");
    while (head) {
        printf("%d", head->val);
        head = head->next;
        if (head) printf(", ");
    }
    printf("]");
}

void runTest(int testNo, const int *input, int size,
             int n, const int *expected, int expectedSize) {

    struct ListNode *head = createList(input, size);

    struct ListNode *result = removeNthFromEnd(head, n);

    bool passed = compareList(result, expected, expectedSize);

    printf("Test %d: %s\n", testNo, passed ? "PASS" : "FAIL");

    if (!passed) {
        printf("  Expected: ");
        printList(createList(expected, expectedSize)); // Display expected separately
        printf("\n  Got:      ");
        printList(result);
        printf("\n");
    }

    freeList(result);
}

int main(void) {
    runTest(1, (int[]){1, 2, 3, 4, 5}, 5,
            2, (int[]){1, 2, 3, 5}, 4);

    runTest(2, (int[]){1}, 1,
            1, (int[]){}, 0);

    runTest(3, (int[]){1, 2}, 2,
            1, (int[]){1}, 1);

    return 0;
}
