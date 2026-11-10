#include <stdlib.h>

#include "simple-iter.c"

int main() {
    // A test just to see how the system behaves
    struct ListNode *nodes = malloc(sizeof(struct ListNode) * 4);
    for (int i = 0; i < 4; ++i) {
        nodes[i].val = i + 1;
        nodes[i].next = nodes + i + 1;
    }
    nodes[3].next = 0;
    struct ListNode *n = swapPairs(nodes);
}
