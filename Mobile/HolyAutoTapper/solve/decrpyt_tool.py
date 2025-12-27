def encrypt_luac(input_path: str, output_path: str, key: bytes = b"a%^*b"):
    """
    luac 파일을 암호화합니다.
    - 처음 33바이트 헤더는 그대로 유지
    - 이후 데이터는 암호화 적용
    
    복호화 순서: 1단계(XOR+NOT) -> 2단계(체인 XOR + 역순)
    암호화 순서: 2단계 역순(역순 + 체인 XOR 역연산) -> 1단계 역순(XOR+NOT)
    """
    HEADER_SIZE = 33
    CHUNK_SIZE = 1024  # 원본 getF와 동일하게 0x400
    
    with open(input_path, "rb") as f:
        data = bytearray(f.read())
    
    if len(data) <= HEADER_SIZE:
        print("파일이 너무 작습니다.")
        return
    
    header = data[:HEADER_SIZE]
    plaintext = data[HEADER_SIZE:]
    
    encrypted = bytearray()
    file_pos = HEADER_SIZE  # 파일 내 실제 위치 (키 인덱스 계산용)
    prev_state = 0  # 체인 복호화의 초기 상태
    
    # 청크 단위로 처리 (복호화 시 청크 단위로 처리되므로)
    offset = 0
    while offset < len(plaintext):
        chunk_size = min(CHUNK_SIZE, len(plaintext) - offset)
        chunk = plaintext[offset:offset + chunk_size]
        
        # ============================================
        # 암호화 (복호화의 역순)
        # ============================================
        
        # 2단계 역연산: 역순 복원 + 체인 XOR 역연산
        # 복호화: state ^= buff[i]; temp[i] = state; 후 역순 복사
        # 암호화: 역순으로 읽어서 체인 XOR 역연산
        
        # 먼저 역순으로 만듦
        reversed_chunk = bytearray(chunk[::-1])
        
        # 체인 XOR 역연산
        # 복호화: state = prev ^ buff[0] ^ buff[1] ^ ... ^ buff[i]
        # 암호화: buff[i] = temp[i] ^ temp[i-1] (temp[-1] = prev_state)
        temp = bytearray(chunk_size)
        for i in range(chunk_size):
            if i == 0:
                temp[i] = reversed_chunk[i] ^ prev_state
            else:
                temp[i] = reversed_chunk[i] ^ reversed_chunk[i - 1]
        
        # 다음 청크를 위한 상태 업데이트
        # 복호화 후 state는 모든 원본 바이트를 XOR한 값
        for b in chunk:
            prev_state ^= b
        
        # 1단계 역연산: XOR + NOT (자기 자신의 역연산)
        # ~(x ^ key) 의 역연산은 ~x ^ key = ~(x ^ key) (NOT과 XOR은 교환 가능)
        # 실제로: encrypt = ~(plain ^ key), decrypt = ~(enc ^ key)
        # 따라서 암호화도 동일: ~(temp ^ key)
        enc_chunk = bytearray(chunk_size)
        for i in range(chunk_size):
            key_idx = (file_pos + i) % 5
            enc_chunk[i] = (~(temp[i] ^ key[key_idx])) & 0xFF
        
        encrypted.extend(enc_chunk)
        offset += chunk_size
        file_pos += chunk_size
    
    # 결과 저장
    with open(output_path, "wb") as f:
        f.write(header)
        f.write(encrypted)
    
    print(f"암호화 완료: {output_path}")
    print(f"원본 크기: {len(data)} bytes")
    print(f"암호화된 크기: {len(header) + len(encrypted)} bytes")


def decrypt_luac(input_path: str, output_path: str, key: bytes = b"a%^*b"):
    """
    암호화된 luac 파일을 복호화합니다 (검증용).
    """
    HEADER_SIZE = 33
    CHUNK_SIZE = 1024
    
    with open(input_path, "rb") as f:
        data = bytearray(f.read())
    
    header = data[:HEADER_SIZE]
    ciphertext = data[HEADER_SIZE:]
    
    decrypted = bytearray()
    file_pos = HEADER_SIZE
    prev_state = 0
    
    offset = 0
    while offset < len(ciphertext):
        chunk_size = min(CHUNK_SIZE, len(ciphertext) - offset)
        chunk = ciphertext[offset:offset + chunk_size]
        
        # 1단계: XOR + NOT 복호화
        temp = bytearray(chunk_size)
        for i in range(chunk_size):
            key_idx = (file_pos + i) % 5
            temp[i] = (~(chunk[i] ^ key[key_idx])) & 0xFF
        
        # 2단계: 체인 XOR 복호화
        state = prev_state
        temp2 = bytearray(chunk_size)
        for i in range(chunk_size):
            state ^= temp[i]
            temp2[i] = state
        
        # 역순으로 복사
        dec_chunk = bytearray(chunk_size)
        for i in range(chunk_size):
            dec_chunk[chunk_size - 1 - i] = temp2[i]
        
        # 다음 청크를 위한 상태 업데이트
        for b in dec_chunk:
            prev_state ^= b
        
        decrypted.extend(dec_chunk)
        offset += chunk_size
        file_pos += chunk_size
    
    with open(output_path, "wb") as f:
        f.write(header)
        f.write(decrypted)
    
    print(f"복호화 완료: {output_path}")


if __name__ == "__main__":
    import sys
    
    if len(sys.argv) < 3:
        print("사용법:")
        print("  암호화: python enc.py encrypt <input.luac> <output.luac>")
        print("  복호화: python enc.py decrypt <input.luac> <output.luac>")
        sys.exit(1)
    
    mode = sys.argv[1]
    input_file = sys.argv[2]
    output_file = sys.argv[3] if len(sys.argv) > 3 else input_file + ".enc"
    
    if mode == "encrypt":
        encrypt_luac(input_file, output_file)
    elif mode == "decrypt":
        decrypt_luac(input_file, output_file)
    else:
        print(f"알 수 없는 모드: {mode}")