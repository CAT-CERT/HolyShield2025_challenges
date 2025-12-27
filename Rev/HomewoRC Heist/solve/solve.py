#!/usr/bin/env python3
import sys
import struct

# ========== Key ==========
enc_master = bytes([
    0x51,0x14,0x38,0x88,0xF7,0xD3,0x5E,0x93,
    0x31,0x6E,0xD4,0xE6,0x7C,0x99,0x3B,0x1C
])
MASTER_MASK = 0x5A

secret_table = bytes([0x5A,0x1F,0xC3,0x77,0x9E,0x2D,0xA1,0x4B, 0x11, 0x22, 0x33, 0x44])

# ========== 롤/로테이트 ==========
def rol8(v, r):
    r &= 7
    return ((v << r) & 0xFF) | ((v & 0xFF) >> (8 - r))

def ror8(v, r):
    r &= 7
    return ((v & 0xFF) >> r) | ((v << (8 - r)) & 0xFF)

# ========== myhash32 (kd의 해시 함수) ==========
def myhash32(data: bytes) -> int:
    # C: uint32_t h = 0x811C9DC5u; for each byte: h ^= b; h *= 0x01000193u; h ^= (h >> 16);
    h = 0x811C9DC5
    for b in data:
        h ^= b
        h = (h * 0x01000193) & 0xFFFFFFFF
        h ^= (h >> 16)
        h &= 0xFFFFFFFF
    return h

# ========== kd_derive_key 재현 ==========
def reconstruct_master(length):
    # C: master[i] = rol8(enc_master[i%enc_master_len], 3) ^ MASTER_MASK
    m = bytearray(length)
    enc_len = len(enc_master)
    for i in range(length):
        b_enc = enc_master[i % enc_len]
        tmp = rol8(b_enc, 3)
        m[i] = tmp ^ MASTER_MASK
    return bytes(m)

def kd_derive_key(salt8: bytes or None, out_len: int) -> bytes:
    if out_len == 0:
        return b''
    master_len = len(enc_master)
    # C used a buffer up to 64 but enc_master_len=16 so fine.
    master = reconstruct_master(master_len)
    out = bytearray(out_len)
    counter = 0
    for pos in range(out_len):
        tmp = bytearray()
        # copy master
        tmp += master[:master_len]
        if salt8:
            tmp += salt8[:8]
        tmp.append(counter & 0xFF)
        h = myhash32(bytes(tmp))
        out[pos] = h & 0xFF
        counter = (counter + 1) & 0xFF
    # zero_mem(master) in C; Python GC will handle it but we'll overwrite local var for caution
    master = b'\x00' * master_len
    return bytes(out)

# ========== opaque_key_dep 재현 ==========
def opaque_key_dep(secret: bytes, seed: int) -> bool:
    # C version:
    # uint32_t acc = seed ^ 0xA5A5A5A5u;
    # for (i=0; i<secret_len; i += 4) {
    #   uint32_t w = 0;
    #   for (b=0;b<4;b++) w = (w << 8) | secret[(i+b) % secret_len];
    #   acc = _rotl(acc ^ w, (int)(i & 31));
    #   acc ^= (acc << 13) | (acc >> 19);
    # }
    # return (acc % 3) == 0;
    acc = (seed ^ 0xA5A5A5A5) & 0xFFFFFFFF
    slen = len(secret)
    i = 0
    while i < slen:
        w = 0
        for b in range(4):
            w = ((w << 8) | secret[(i + b) % slen]) & 0xFFFFFFFF
        # rotl acc ^ w by (i & 31)
        r = i & 31
        acc_x = (acc ^ w) & 0xFFFFFFFF
        acc = ((acc_x << r) | (acc_x >> (32 - r))) & 0xFFFFFFFF
        # acc ^= (acc << 13) | (acc >> 19)
        tmp = ((acc << 13) | (acc >> 19)) & 0xFFFFFFFF
        acc ^= tmp
        acc &= 0xFFFFFFFF
        i += 4
    return (acc % 3) == 0

# ========== cc_init_state 재현 ==========
def cc_init_state(key: bytes):
    # C: KeyLen = min(keylen,256); KeyBytes[0..KeyLen-1] = key
    keylen = min(len(key), 256)
    key_bytes = bytearray(256)
    key_bytes[:keylen] = key[:keylen]

    Sbox = bytearray(range(256))
    j = 0
    idx = 0

    while idx < 256:
        i = idx
        k = key_bytes[i % keylen] if keylen > 0 else 0
        add = ((i * i) >> 3) & 0xFF
        if opaque_key_dep(secret_table, 0x1234):
            j = (j + Sbox[i] + k + add) & 0xFF
            # swap Sbox[i] and Sbox[j]
            Sbox[i], Sbox[j] = Sbox[j], Sbox[i]
        else:
            j = (j + Sbox[i] + k + ((i * 7) & 0xFF)) & 0xFF
            # swap Sbox[(i+1)&0xFF] and Sbox[j]
            idx2 = (i + 1) & 0xFF
            Sbox[idx2], Sbox[j] = Sbox[j], Sbox[idx2]
        idx += 1
    Sbox_init = bytes(Sbox)
    return Sbox_init

# ========== cc_crypt_buffer 재현 (암호화와 복호화가 동일 XOR) ==========
def cc_crypt_buffer(buf: bytearray, Sbox_init: bytes):
    localS = bytearray(Sbox_init)  # copy
    i = 0
    j = 0
    buflen = len(buf)
    for pos in range(buflen):
        i = (i + 1) & 0xFF
        j = (j + localS[i]) & 0xFF

        # C: uses opaque_key_dep(secret_table, sizeof(secret_table), (uint32_t)pos)
        if opaque_key_dep(secret_table, pos):
            # swap localS[i] and localS[j]
            localS[i], localS[j] = localS[j], localS[i]
        else:
            # swap localS[j] and localS[i] (same as above)
            localS[j], localS[i] = localS[i], localS[j]

        raw = localS[(localS[i] + localS[j]) & 0xFF]
        outv = ror8(raw, (i + j) & 7)
        buf[pos] ^= outv

# ========== 파일 포맷 파서 및 복호화 엔트리 ==========
def decrypt_file(in_path: str, out_path: str):
    with open(in_path, "rb") as f:
        data = f.read()

    if len(data) < 4 + 1 + 3 + 8 + 4:
        raise ValueError("file too small to be valid")

    # header: magic 'CR4F' (4), then version byte, then 3 reserved bytes, then salt (8), then length (u32 little endian)
    magic = data[0:4]
    if magic != b'CR4F':
        raise ValueError("invalid magic: expected 'CR4F'")

    # bytes 4..7 are version+3 reserved
    salt = data[8:16]
    # length is little-endian uint32 at offset 16..19
    datalen = struct.unpack_from("<I", data, 16)[0]
    payload = bytearray(data[20:20 + datalen])
    if len(payload) != datalen:
        raise ValueError("payload length mismatch or truncated")

    # derive key (32 bytes in C code)
    key = kd_derive_key_py(salt, 32)

    # init cc state
    Sbox_init = cc_init_state(key)

    # decrypt (same as encrypt)
    cc_crypt_buffer(payload, Sbox_init)

    # write output
    with open(out_path, "wb") as f:
        f.write(payload)
    print(f"Decrypted to {out_path}")

# wrapper exposing kd_derive_key implemented above
def kd_derive_key_py(salt8: bytes, out_len: int) -> bytes:
    return kd_derive_key(salt8, out_len)

def main():
    if len(sys.argv) != 3:
        print("Usage: python3 decryptor.py input.enc output.docx")
        return
    in_path = sys.argv[1]
    out_path = sys.argv[2]
    try:
        decrypt_file(in_path, out_path)
    except Exception as e:
        print("Error:", e)

if __name__ == "__main__":
    main()
