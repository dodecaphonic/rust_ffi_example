#include <stdio.h>
#include <math.h>

struct magic_linked_list {
        int value;
        struct magic_linked_list *next;
};

long magic_multiply(struct magic_linked_list *list) {
        if (!list) return 0;

        long result = list->value;
        struct magic_linked_list *next, *current = list;

        while ((next = current->next)) {
                result *= next->value;
                current = next;
        }

        return result;
}

double magic_pow(double n, double p) {
        return pow(n, p);
}

int main() {
        struct magic_linked_list n0, n1, n2;
        n0.value = 10;
        n1.value = 20;
        n2.value = 3;

        n0.next = &n1;
        n1.next = &n2;
        n2.next = NULL;

        printf("%ld\n", magic_multiply(&n0));
}
