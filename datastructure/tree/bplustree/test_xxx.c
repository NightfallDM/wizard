#include <stdio.h>
#include <stddef.h>

struct xxx {
    int x1;
    int x2;
    int x3;
    long x4;
    char x5;
};

int main() {
    printf("x1: %ld, x2: %ld, x3: %ld, x4: %ld, x5: %ld", offsetof(struct xxx, x1), offsetof(struct xxx, x2), offsetof(struct xxx, x3), offsetof(struct xxx, x4), offsetof(struct xxx, x5));
    printf("\n%ld\n", sizeof(long unsigned int));
}