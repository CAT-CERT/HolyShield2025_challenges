#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

const char *target = "happysmile";
int ck = 0;

int setup()
{
    setbuf(stdin, NULL);
    setbuf(stdout, NULL);
    return 0;
}

int board() {
    puts("===================================");
    puts("=    Welcome to AvoTapo World!    =");
    puts("===================================");

    return 0;
}

int check(char *id) {
    for (int i = 0; i < 10; i++) {
        if (id[i] != target[i]) {
            return -1;
        }
    }
    return 0;
}

void menu()
{
    puts("\n\n===========================");
    puts("| What do you want to do? |");
    puts("===========================");
    puts("1. Test");
    puts("2. PWN");
    puts("3. Exit");
}

void test()
{
    system("./testing");
    printf("Test Completed. See ./logfile");
}

int pwn()
{
    unsigned long addr;
    unsigned long value;

    scanf("%ld", &addr);
    scanf("%ld", &value);

    if(ck) {
        unsigned long *ptr = addr;
        *ptr = (unsigned long)value;
    }
    else {
        char *ptr = (char *)addr;
        *ptr = (char)value;
    }
    ck += 1;

    return 0;
}

int administrator()
{
    int opt;

    menu();
    printf(">> ");
    scanf("%d", &opt);

    if(opt == 1) {
        printf("Testing Camera...");
        test();
    }

    else if(opt == 2) {
        printf("pwn pwn!\n");
        pwn();
    }

    else if(opt == 3) {
        puts("Good Bye");
        exit(0);
    }
    
    else {
        puts("Invalid Options");
        exit(1);
    }

    return 0;
}

int last()
{
    puts("Good Bye");
}

void (*run_last)(void) __attribute__((section(".fini_array"))) = (void (*)())last;

int main()
{
    char id[0x40];

    setup();
    board();

    printf("Enter ID : ");
    scanf("%63[^\n]s", id);

    if(check(id)) {
        puts("Not Authorized");
        exit(1);
    }

    printf("hello ");
    printf(id);
    
    administrator();
    
    asm volatile (
        "xor %%rax, %%rax\n\t"
        "xor %%rbx, %%rbx\n\t"
        "xor %%rcx, %%rcx\n\t"
        "xor %%rdx, %%rdx\n\t"
        "xor %%rsi, %%rsi\n\t"
        "xor %%r8, %%r8\n\t"
        "xor %%r9, %%r9\n\t"
        "xor %%r10, %%r10\n\t"
        "xor %%r11, %%r11\n\t"
        "xor %%r12, %%r12\n\t"
        "xor %%r13, %%r13\n\t"
        "xor %%r14, %%r14\n\t"
        "xor %%r15, %%r15\n\t"
        :
        :
        : "rax", "rbx", "rcx", "rdx", "rsi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
    );
    
    puts("Exiting The Program.");
    return 0;
}