// Scribbles while reading section 2.1.1 Hexadecimal Notation of Bryant & O'Hallaron

#include "stdio.h"


void print_hex_repr(const size_t size, const void * const ptr) {
    /**
     * Print Hexadecimal representation of number stored at $size bytes starting at $ptr.
     * */
    unsigned char *b = (unsigned char*) ptr;
    printf("hexadecimal: Ox");
    for (int i = size-1; i >= 0; i--) { // loop over bytes
        printf("%02x", b[i]);
    }
    puts(";");
}


void print_binary_repr(const size_t size, const void * const ptr) {
    unsigned char *b = (unsigned char*) ptr;
    unsigned char curr_byte;

    printf("binary: ");

    int i, j;
    for (i = size-1; i >= 0; i--) { // loop over bytes
        for (j = 7; j >= 0; j--) { // loop over current byte
            curr_byte = (b[i] >> j) & 1;
            printf("%u", curr_byte);
        }
    }
    puts(";");
}


int main() {
    // CHARS
    unsigned char A_U[] = {0, 1, 2, 32, 129, 192, 255};
    printf("UNSIGNED CHAR (%ld bytes)\n", sizeof(A_U[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %d\n", A_U[i]);
        print_binary_repr(sizeof(A_U[i]), &A_U[i]);
        print_hex_repr(sizeof(A_U[i]), &A_U[i]);
        printf(".....\n");
    }
    printf("-----\n");
    char A_S[] = {-128, 0, 1, 2, 32, 127, 128};
    printf("SIGNED CHAR (%ld bytes)\n", sizeof(A_S[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %d\n", A_S[i]);
        print_binary_repr(sizeof(A_S[i]), &A_S[i]);
        print_hex_repr(sizeof(A_S[i]), &A_S[i]);
        printf(".....\n");
    }
    // Interesting Notes
    // * 128 overflows and rolls over to -128.
    printf("-----\n");
    printf("-----\n");

    // SHORTS
    unsigned short B_U[] = {0, 1, 32, 5198, 123, 65535, 65536};
    printf("\nSHORTS (%ld bytes)\n", sizeof(B_U[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %d\n", B_U[i]);
        print_binary_repr(sizeof(B_U[i]), &B_U[i]);
        print_hex_repr(sizeof(B_U[i]), &B_U[i]);
        printf(".....\n");
    }
    // Interesting Notes:
    // GCC actually highlights the overflow
    // gcc -o target/exec src/main.c
    // src/main.c: In function ‘main’:
    // src/main.c:60:57: warning: unsigned conversion from ‘int’ to ‘short unsigned int’ changes value from ‘65536’ to ‘0’ [-Woverflow]
    //    60 |     unsigned short B_U[] = {0, 1, 32, 5198, 123, 65535, 65536};
    //       |

    short B_S[] = {0, 1, 2, 3, 123, 4096, 8192};
    printf("\nUNSIGNED SHORT (%ld bytes)\n", sizeof(B_S[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %d\n", B_S[i]);
        print_binary_repr(sizeof(B_S[i]), &B_S[i]);
        print_hex_repr(sizeof(B_S[i]), &B_S[i]);
        printf(".....\n");
    }

    unsigned int C[] = {0, 1, 2, 32, 123, 65536, 4294967295};
    printf("\nUNSIGNED INT (%ld bytes)\n", sizeof(C[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %u\n", C[i]);
        print_binary_repr(sizeof(C[i]), &C[i]);
        print_hex_repr(sizeof(C[i]), &C[i]);
        printf(".....\n");
    }
    printf("-----\n");
    printf("-----\n");

    unsigned long int D[] = {0, 1, 2, 32, 65536, 4294967295, 18446744073709551615};
    printf("\nUNSIGNED LONG (%ld bytes)\n", sizeof(D[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %lu\n", D[i]);
        print_binary_repr(sizeof(D[i]), &D[i]);
        print_hex_repr(sizeof(D[i]), &D[i]);
        printf(".....\n");
    }

    long int E[] = {0, 1, 2, 32, 65536, 4294967295, 18446744073709551615};
    printf("\nSIGNED LONG (%ld bytes)\n", sizeof(E[0]));
    for (int i=0; i < 7; i++) {
        printf("decimal: %ld\n", E[i]);
        print_binary_repr(sizeof(E[i]), &E[i]);
        print_hex_repr(sizeof(E[i]), &E[i]);
        printf(".....\n");
    }
}
