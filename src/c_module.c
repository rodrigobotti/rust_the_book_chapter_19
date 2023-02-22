#include<stdio.h>

int print_and_return(int x) {
    printf("-- [C function] --\n");
    printf("value is %d\n", x);
    printf("-- [end of C function] --\n");
    return x;
}