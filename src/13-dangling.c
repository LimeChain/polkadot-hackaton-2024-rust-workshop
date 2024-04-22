#include <stdio.h>

int *dangling();
void disturb();

int main() {
    int *ptr = dangling();
    disturb();
    printf("%d\n", *ptr);
}

void disturb() {
    int a = 6;
}

int *dangling() {
    int a = 5;
    return &a;
}
