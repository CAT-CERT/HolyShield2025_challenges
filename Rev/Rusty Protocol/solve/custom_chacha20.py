import struct

# ====== 고정 파라미터 (암호화 때와 동일해야 함) ======
CONST = b"expand 32-byte k"
KEY_STR = "j_k0o'g_oNnk_Gr_k3hAvUg_7w!D_0o3"
KEY = KEY_STR.encode("utf-8")

NONCE = b"1029384756aa"[:12]   # 암호화 때 사용한 Nonce와 동일해야 함
INITIAL_COUNTER = 1              # 암호화 때 사용한 Counter와 동일해야 함

# ====== 유틸 ======
def rotl32(x: int, r: int) -> int:
    x &= 0xFFFFFFFF
    return ((x << r) & 0xFFFFFFFF) | (x >> (32 - r))

def u32_le(b: bytes) -> int:
    return struct.unpack("<I", b)[0]

def to_le_u32_words(b: bytes) -> list[int]:
    assert len(b) % 4 == 0
    return [u32_le(b[i:i+4]) for i in range(0, len(b), 4)]

def words_to_le_bytes(words: list[int]) -> bytes:
    return b"".join(struct.pack("<I", w & 0xFFFFFFFF) for w in words)

# ====== Quarter Round ======
def quarter_round(a: int, b: int, c: int, d: int) -> tuple[int,int,int,int]:
    a = (a + b) & 0xFFFFFFFF; d ^= a; d = rotl32(d, 16)
    c = (c + d) & 0xFFFFFFFF; b ^= c; b = rotl32(b, 12)
    a = (a + b) & 0xFFFFFFFF; d ^= a; d = rotl32(d,  8)
    c = (c + d) & 0xFFFFFFFF; b ^= c; b = rotl32(b,  7)
    return a, b, c, d

def qr_idx(st: list[int], i: int, j: int, k: int, l: int) -> None:
    A, B, C, D = st[i], st[j], st[k], st[l]
    A, B, C, D = quarter_round(A, B, C, D)
    st[i], st[j], st[k], st[l] = A, B, C, D

# ====== 라운드 스케줄 ======
def round_column(st: list[int]) -> None:
    qr_idx(st, 0, 4,  8, 12)
    qr_idx(st, 1, 5,  9, 13)
    qr_idx(st, 2, 6, 10, 14)
    qr_idx(st, 3, 7, 11, 15)

def round_diagonal(st: list[int]) -> None:
    qr_idx(st, 0, 5, 10, 15)
    qr_idx(st, 1, 6, 11, 12)
    qr_idx(st, 2, 7,  8, 13)
    qr_idx(st, 3, 4,  9, 14)

def round_custom(st: list[int]) -> None:
    qr_idx(st,  6,  3,  1,  0)
    qr_idx(st, 10,  7,  4,  2)
    qr_idx(st, 13, 11,  8,  5)
    qr_idx(st, 15, 14, 12,  9)

# ====== 블록 함수 ======
def build_state(key: bytes, counter: int, nonce: bytes) -> list[int]:
    if len(key) != 32:
        raise ValueError("key must be 32 bytes")
    if len(nonce) != 12:
        raise ValueError("nonce must be 12 bytes")

    const_words = to_le_u32_words(CONST)
    key_words   = to_le_u32_words(key)
    ctr_words   = [counter & 0xFFFFFFFF]
    nonce_words = to_le_u32_words(nonce)
    return const_words + key_words + ctr_words + nonce_words

def custom_chacha20_block(key: bytes, counter: int, nonce: bytes) -> bytes:
    x = build_state(key, counter, nonce)
    st = x.copy()

    for _ in range(10):  # Triple round ×10 = 30라운드
        round_column(st)
        round_diagonal(st)
        round_custom(st)

    for i in range(16):
        st[i] = (st[i] + x[i]) & 0xFFFFFFFF

    return words_to_le_bytes(st)  # 64바이트 키스트림

def keystream(key: bytes, nonce: bytes, counter: int, nbytes: int) -> bytes:
    out = bytearray()
    ctr = counter & 0xFFFFFFFF
    while len(out) < nbytes:
        block = custom_chacha20_block(key, ctr, nonce)
        take = min(nbytes - len(out), 64)
        out += block[:take]
        ctr = (ctr + 1) & 0xFFFFFFFF
    return bytes(out)

def decrypt(key: bytes, nonce: bytes, counter: int, ciphertext: bytes) -> bytes:
    ks = keystream(key, nonce, counter, len(ciphertext))
    return bytes(c ^ k for c, k in zip(ciphertext, ks))

# ====== 메인 ======
if __name__ == "__main__":
    cipher_hex = input("복호화할 암호문(hex) 입력: ").strip()
    ciphertext = bytes.fromhex(cipher_hex)

    plaintext = decrypt(KEY, NONCE, INITIAL_COUNTER, ciphertext)
    try:
        print("복호화 결과:", plaintext.decode("utf-8"))
    except UnicodeDecodeError:
        print("복호화 결과(바이너리):", plaintext.hex())
