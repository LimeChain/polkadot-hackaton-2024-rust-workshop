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
