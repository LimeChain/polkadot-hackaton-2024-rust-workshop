#include <stdio.h>

int *dangling() {
    int a = 5;
    return &a;
}

void disturb() {
    int a = 6;
}

int main() {
    int *ptr = dangling();
    disturb();
    printf("%d\n", *ptr);
}

// To show bug compile with `clang -O0 14-dangling.c -o 14-dangling`
