
#define _CRT_SECURE_NO_WARNINGS

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define STACK_SIZE 256
#define INPUT_SIZE 64

typedef struct {
    uint64_t stack[STACK_SIZE];
    int sp; // 스택 포인터
    unsigned char* code; // shield.bin opcode
    size_t code_size;
    uint64_t result; // 최종 결과 저장
    unsigned char input[INPUT_SIZE];
} VM;

enum OPCODES {
    OP_PUSH = 0x01,
    OP_ADD = 0x03,
    OP_SUB = 0x04,
    OP_XOR = 0x05,
    OP_MUL = 0x06,
    OP_AND = 0x07,
    OP_OR = 0x08,
    OP_ROL = 0x09,
    OP_SHR = 0x0A,
    OP_END = 0xFF
};

void push(VM* vm, uint64_t val) {
    if (vm->sp < STACK_SIZE) {
        vm->stack[vm->sp++] = val;
    }
}

uint64_t pop(VM* vm) {
    if (vm->sp > 0) {
        return vm->stack[--vm->sp];
    }
    return 0;
}

void vm_run(VM* vm) {
    size_t ip = 0; // instruction pointer

    while (ip < vm->code_size) {
        unsigned char opcode = vm->code[ip++];
        switch (opcode) {
        case OP_PUSH: {
            uint8_t idx = vm->code[ip++];
            uint64_t val = *(uint64_t*)(vm->input + idx);
            push(vm, val);
            break;
        }
        case OP_ADD: {
            uint64_t a = pop(vm);
            uint64_t b = pop(vm);
            push(vm, a + b);
            break;
        }
        case OP_SUB: {
            uint64_t a = pop(vm);
            uint64_t b = pop(vm);
            push(vm, a - b);
            break;
        }
        case OP_XOR: {
            uint64_t a = pop(vm);
            uint64_t b = pop(vm);
            push(vm, a ^ b);
            break;
        }
        case OP_MUL: {
            uint64_t a = pop(vm);
            uint64_t b = pop(vm);
            push(vm, a * b);
            break;
        }
        case OP_AND: {
            uint64_t a = pop(vm);
            uint64_t b = pop(vm);
            push(vm, a & b);
            break;
        }
        case OP_OR: {
            uint64_t a = pop(vm);
            uint64_t b = pop(vm);
            push(vm, a | b);
            break;
        }
        case OP_ROL: {
            uint8_t shift = vm->code[ip++];
            uint64_t val = pop(vm);
            push(vm, (val << shift) | (val >> (64 - shift)));
            break;
        }
        case OP_SHR: {
            uint8_t shift = vm->code[ip++];
            uint64_t val = pop(vm);
            push(vm, val >> shift);
            break;
        }
        case OP_END:
            vm->result = pop(vm);
            return;
        default:
            printf("Unknown opcode: 0x%02X\n", opcode);
            return;
        }
    }
}

int main() {
    VM vm;
    vm.sp = 0;

    FILE* f = fopen("shield.bin", "rb");
    if (!f) { perror("shield.bin"); return 1; }
    fseek(f, 0, SEEK_END);
    vm.code_size = ftell(f);
    fseek(f, 0, SEEK_SET);
    vm.code = malloc(vm.code_size);
    fread(vm.code, 1, vm.code_size, f);
    fclose(f);

    printf("Input 64 bytes (hex): ");
    for (int i = 0; i < INPUT_SIZE; i++) {
        unsigned int val;
        if (scanf("%02x", &val) != 1) { printf("bad input\n"); return 1; }
        vm.input[i] = val;
    }

    vm_run(&vm);

    const uint64_t expected_value = 0x77488b4302b4689b; // 생성된 expected value
    if (vm.result == expected_value) {
        printf("correct\n");
    }
    else {
        printf("wrong\n");
    }

    free(vm.code);
    return 0;
}
