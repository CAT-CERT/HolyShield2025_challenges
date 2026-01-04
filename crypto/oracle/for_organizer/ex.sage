# solve.sage
import hashlib
import binascii

# ----------------------------------------------------------------
# 1. Rust 로직 재구현 (파라미터 복구)
# ----------------------------------------------------------------

def rotate_right_u64(n, shift):
    # 64비트 정수 회전 연산
    n = int(n) & 0xFFFFFFFFFFFFFFFF
    return ((n >> shift) | (n << (64 - shift))) & 0xFFFFFFFFFFFFFFFF

def get_modulus_n():
    print("[*] Reconstructing Modulus N...")
    raw_parts = [
        0x1234567890abcdef, 0x1122334455667788, 0x9988776655443322, 0xaabbccddeeff0011,
        0x0000000000000001, 0xcafebabecafebabe, 0xfeedfacefeedface, 0xdeadbeefdeadbeef,
        0x0102030405060708, 0x090a0b0c0d0e0f10, 0x1011121314151617, 0x18191a1b1c1d1e1f,
        0x2021222324252627, 0x28292a2b2c2d2e2f, 0x3031323334353637, 0x38393a3b3c3d3e3f
    ]
    
    pressure = 0
    for i, part in enumerate(raw_parts):
        # SageMath에서 XOR 연산자는 '^^'
        twist = rotate_right_u64(i, 2) ^^ 0x5555555555555555
        val = part ^^ twist
        pressure = pressure + (val << (i * 64))
    return pressure

def find_seed():
    print("[*] Finding Seed (Brute-force)...")
    salt = b"OrAcLe"
    # Rust 코드와 동일한 조건: SHA256의 상위 3바이트가 000000
    # 보통 CTF 문제는 앞쪽 범위에서 나오도록 설계됨
    for i in range(0xffffffff): 
        b_seed = int(i).to_bytes(4, 'big')
        h = hashlib.sha256(b_seed + salt).digest()
        if h[0] == 0 and h[1] == 0 and h[2] == 0:
            print(f"[+] Found Seed: {i}")
            return i
    return None

def generate_prefix(seed):
    print("[*] Generating Prefix...")
    chronicle = []
    magic_constant = 0x1337_BEEF
    for i in range(88):
        # 32비트 오버플로우 처리
        val_add = (seed + i) & 0xFFFFFFFF
        val_mul = (val_add * magic_constant) & 0xFFFFFFFF
        val = val_mul ^^ 0xFF  # XOR
        chronicle.append((val >> 24) & 0xFF)
    return bytes(chronicle)

# ----------------------------------------------------------------
# 2. Coppersmith Attack (복호화 수행)
# ----------------------------------------------------------------

def solve():
    # 1. 파라미터 준비 (N, Seed, Prefix)
    N = get_modulus_n()
    seed = find_seed()
    
    if seed is None:
        print("[-] Seed not found. Check the range or implementation.")
        return

    prefix_bytes = generate_prefix(seed)
    prefix_int = Integer(int(binascii.hexlify(prefix_bytes), 16))
    e = 3

    # 2. 암호문 입력 받기
    print("\n" + "="*50)
    print("Paste the HEX Output from the Rust program:")
    try:
        c_hex = input("Ciphertext > ").strip()
        C = Integer(int(c_hex, 16))
    except:
        print("[-] Invalid Hex String")
        return

    print("\n[*] Starting Coppersmith's Stereotyped Message Attack...")
    
    # 다항식 링(Polynomial Ring) 설정
    P = PolynomialRing(Zmod(N), implementation='NTL', names='x')
    (x,) = P._first_ngens(1)

    # Flag의 길이를 정확히 모르므로, 가능한 길이(20~42 바이트)를 순회하며 시도
    # N이 1024비트이고 e=3이므로, 복구 가능한 미지수(Flag)의 최대 크기는
    # 약 1024 / 3 = 341비트 (약 42바이트)입니다.
    found_flag = False
    
    for flag_len in range(10, 45):
        shift_bits = flag_len * 8
        
        # 방정식 구성: (Prefix * 2^shift + x)^3 - C ≡ 0 (mod N)
        # 여기서 ^는 SageMath의 거듭제곱 연산
        f = (prefix_int * (2^shift_bits) + x)^e - C
        
        # 해의 상한선 설정 (Flag는 flag_len 바이트 크기이므로)
        upper_bound = 2**(shift_bits)
        
        try:
            # small_roots를 사용하여 작은 해(Flag) 탐색
            # epsilon: 탐색 정밀도 (작을수록 느리지만 성공 확률 높음)
            roots = f.small_roots(X=upper_bound, beta=1.0, epsilon=0.03)
            
            if roots:
                # 해를 찾음
                root = Integer(roots[0])
                flag_hex = hex(root)[2:]
                
                # 짝수 길이 보정
                if len(flag_hex) % 2 != 0:
                    flag_hex = '0' + flag_hex
                    
                try:
                    flag_str = binascii.unhexlify(flag_hex).decode()
                    print(f"\n[SUCCESS] Flag found (len={flag_len}): {flag_str}")
                    found_flag = True
                    break
                except:
                    # 디코딩 실패 시 (쓰레기 값일 경우 무시)
                    continue
        except:
            continue
            
    if not found_flag:
        print("\n[-] Failed to recover the flag.")
        print("    Possible reasons:")
        print("    1. The flag is too long (> 42 bytes).")
        print("    2. The input ciphertext is incorrect.")

# 실행
if __name__ == "__main__":
    solve()