# ===== 고정 입력값() =====
KEY_ASCII      = b"i_7H1nk_U'v3_DoNe_QUiTe_@_LO7_:)"  # 32B ASCII Key
NONCE_HEX      = "000102030405060708090a0b0c0d0e0f"   # 16B Nonce (hex)
CIPHERTEXT_HEX = "3c9b8cab45eaade170fafcc945b6abb7a6fe989944c0daa0dc224e71010f398364344d56d8f3b1722e3c"

# ===== 유틸 =====
def rol32(x: int, r: int) -> int:
    x &= 0xFFFFFFFF
    return ((x << r) & 0xFFFFFFFF) | (x >> (32 - r))

def to_u32_le_words(b: bytes):
    assert len(b) % 4 == 0
    return [int.from_bytes(b[i:i+4], 'little') for i in range(0, len(b), 4)]

def from_u32_le_words(ws):
    return b"".join((w & 0xFFFFFFFF).to_bytes(4, "little") for w in ws)

def cham_key_schedule_128_256(key32: bytes):
    K = to_u32_le_words(key32)  # 8 words
    rk_even = [(Ki ^ rol32(Ki,1) ^ rol32(Ki,8))  & 0xFFFFFFFF for Ki in K]
    rk_odd  = [(Ki ^ rol32(Ki,1) ^ rol32(Ki,11)) & 0xFFFFFFFF for Ki in K]
    return rk_even, rk_odd

def cham128_256_encrypt_block(key32: bytes, block16: bytes, rounds: int = 120) -> bytes:
    rk_even, rk_odd = cham_key_schedule_128_256(key32)
    S0, S1, S2, S3 = to_u32_le_words(block16)
    for i in range(rounds):
        if (i & 1) == 0:
            t = (rol32(S1, 1) ^ rk_even[i % 8]) & 0xFFFFFFFF
            newS = rol32(((S0 ^ i) + t) & 0xFFFFFFFF, 8)
        else:
            t = (rol32(S1, 8) ^ rk_odd[i % 8]) & 0xFFFFFFFF
            newS = rol32(((S0 ^ i) + t) & 0xFFFFFFFF, 1)
        S0, S1, S2, S3 = S1, S2, S3, newS
    return from_u32_le_words([S3, S2, S1, S0])

def cham_ctr_decrypt_method2(key32: bytes, nonce16: bytes, ciphertext: bytes) -> bytes:
    if len(nonce16) != 16:
        raise ValueError("Nonce는 16바이트")
    n12 = nonce16[:12]
    out = bytearray()
    blocks = [ciphertext[i:i+16] for i in range(0, len(ciphertext), 16)]
    for idx, C in enumerate(blocks, start=1):
        cb = n12 + idx.to_bytes(4, 'big')               # CB_i = Nonce[0..11] || BE32(i)
        S  = cham128_256_encrypt_block(key32, cb, 120)  # 16B keystream
        out += bytes(c ^ s for c, s in zip(C, S[:len(C)]))
    return bytes(out)

def main():
    key = KEY_ASCII
    nonce = bytes.fromhex(NONCE_HEX)
    ct = bytes.fromhex(CIPHERTEXT_HEX)

    pt = cham_ctr_decrypt_method2(key, nonce, ct)

    try:
        print(pt.decode("utf-8"))
    except UnicodeDecodeError:
        print(pt.hex())

if __name__ == "__main__":
    main()
