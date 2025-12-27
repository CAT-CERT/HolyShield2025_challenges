# make_enc_master.py
# Usage: python3 make_enc_master.py master_hexstring
# Example: python3 make_enc_master.py 00112233445566778899AABBCCDDEEFF

import sys
def rol8(v, r):
    r &= 7
    return ((v << r) | (v >> (8 - r))) & 0xFF

def obf_byte(b, mask=0x5A):
    return ((b ^ mask) >> 3) | ((b ^ mask) << 5) & 0xFF  # ROR by 3 implemented via rol/ror; simpler: ror(b^mask,3)

def ror8(v, r):
    r &= 7
    return ((v >> r) | ((v << (8 - r)) & 0xFF)) & 0xFF

def obf(b, mask=0x5A):
    # b_enc = ROR(b_plain ^ mask, 3)
    return ror8(b ^ mask, 3)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: make_enc_master.py <hexbytes>")
        sys.exit(1)
    s = sys.argv[1]
    if len(s) % 2 != 0:
        print("hex length must be even")
        sys.exit(2)
    out = []
    for i in range(0, len(s), 2):
        b = int(s[i:i+2], 16)
        out.append(obf(b))
    print(", ".join("0x%02X" % x for x in out))
