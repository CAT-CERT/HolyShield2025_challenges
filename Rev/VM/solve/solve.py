from vm_defs import *
from functools import reduce
import math

expected = bytes([
    0xA3, 0x3F, 0x3D, 0xB9, 0xD5, 0xE3, 0xD9, 0x81,
    0x3D, 0x8B, 0xC5, 0xB7, 0x73, 0xDD, 0x0E, 0xFF,
    0x7E, 0xEB, 0x0B, 0x04, 0x4F, 0x6F, 0x01, 0xDE,
    0x62, 0xE6, 0x01, 0x0B, 0xE6, 0xEC, 0xEE, 0x89,
    0x96, 0xD0, 0xDF, 0xC5, 0xC1, 0x21, 0xD0, 0x70,
    0x95, 0x18, 0xB7, 0x07, 0x2C, 0xC1, 0xC2, 0xF8,
    0x16, 0x09, 0x51, 0x96, 0x9C, 0x3D, 0x28, 0x36,
    0x64, 0x83, 0xE5, 0x03, 0xA9, 0x3D, 0x3D, 0x1B
])

CHUNK_SIZE = 16
NUM_CHUNKS = 4

def rol8(v, s):
    return ((v << s) | (v >> (8-s))) & 0xFF

def ror8(v, s):
    return ((v >> s) | (v << (8-s))) & 0xFF

def modinv(a, mod):
    """a의 mod 역원 계산"""
    g, x, _ = extended_gcd(a, mod)
    if g != 1:
        raise Exception(f"No modular inverse for {a} mod {mod}")
    return x % mod

def extended_gcd(a, b):
    if a == 0:
        return (b, 0, 1)
    else:
        g, y, x = extended_gcd(b % a, a)
        return (g, x - (b // a) * y, y)

def inverse_ops(v, imm_ops):
    for op, imm in reversed(imm_ops):
        if op == OP_ADD:
            v = bytes((x - imm) & 0xFF for x in v)
        elif op == OP_SUB:
            v = bytes((x + imm) & 0xFF for x in v)
        elif op == OP_XOR:
            v = bytes((x ^ imm) & 0xFF for x in v)
        elif op == OP_ROL:
            v = bytes(ror8(x, imm & 7) for x in v)
        elif op == OP_ROR:
            v = bytes(rol8(x, imm & 7) for x in v)
        elif op == OP_MUL:
            inv = modinv(imm, 256)
            v = bytes((x * inv) & 0xFF for x in v)
    return v

def solve():
    flag_chunks = [b''] * NUM_CHUNKS

    imm_ops_chunks = [
        [(OP_XOR, 0x11), (OP_ADD, 0x04), (OP_ROL, 0x02), (OP_SUB, 0x05), (OP_MUL, 0x03), (OP_XOR, 0x44), (OP_ADD, 0x33), (OP_ROR, 0x01)],  # chunk 0
        [(OP_SUB, 0x05), (OP_ROL, 0x01), (OP_ADD, 0x22), (OP_XOR, 0x13), (OP_MUL, 0x07), (OP_ROR, 0x02), (OP_SUB, 0x09), (OP_ADD, 0x17)],  # chunk 1
        [(OP_MUL, 0x07), (OP_XOR, 0x33), (OP_ADD, 0x11), (OP_SUB, 0x25), (OP_ROL, 0x03), (OP_XOR, 0x55), (OP_ROR, 0x01), (OP_MUL, 0x03)],  # chunk 2
        [(OP_ADD, 0x44), (OP_ROR, 0x02), (OP_SUB, 0x10), (OP_XOR, 0x66), (OP_MUL, 0x05), (OP_ADD, 0x18), (OP_ROL, 0x01), (OP_XOR, 0x77)]   # chunk 3
    ]

    for chunk_id in range(NUM_CHUNKS):
        start = chunk_id * CHUNK_SIZE
        end = start + CHUNK_SIZE
        chunk_expected = expected[start:end]
        flag_chunks[chunk_id] = inverse_ops(chunk_expected, imm_ops_chunks[chunk_id])

    flag = b''.join(flag_chunks)
    print("[+] FLAG:", flag.decode(errors="ignore"))
    print("[+] FLAG HEX:", flag.hex())
    return flag

if __name__ == "__main__":
    solve()
