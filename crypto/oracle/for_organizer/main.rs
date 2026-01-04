use num_bigint::{BigInt, Sign};
use num_traits::Zero;
use sha2::{Sha256, Digest};
use std::io;

// [Anti-AI] 암호학 용어 숨기기
struct ClockworkEngine {
    // 경고 방지를 위해 '_'(underscore) 접두사 사용 또는 allow attribute 사용
    #[allow(dead_code)]
    gear_ratios: Vec<u64>, 
}

impl ClockworkEngine {
    // 복잡한 수식으로 N 생성 (리버싱 포인트 1)
    fn ignite_steam_engine() -> BigInt {
        // [수정됨] 타입을 Vec<u64>로 명시하여 컴파일러가 숫자를 i32가 아닌 u64로 인식하게 함
        let raw_parts: Vec<u64> = vec![
            0x1234567890abcdef, 0x1122334455667788, 0x9988776655443322, 0xaabbccddeeff0011,
            0x0000000000000001, 0xcafebabecafebabe, 0xfeedfacefeedface, 0xdeadbeefdeadbeef,
            0x0102030405060708, 0x090a0b0c0d0e0f10, 0x1011121314151617, 0x18191a1b1c1d1e1f,
            0x2021222324252627, 0x28292a2b2c2d2e2f, 0x3031323334353637, 0x38393a3b3c3d3e3f
        ];

        let mut pressure = BigInt::zero();
        for (i, &part) in raw_parts.iter().enumerate() {
            let mut val = BigInt::from(part);
            
            // AI가 예측하기 힘든 비트 연산 (Rotate & XOR)
            // twist 연산을 통해 값을 변형시킴
            let twist = (i as u64).rotate_right(2) ^ 0x5555555555555555;
            val = val ^ BigInt::from(twist);
            
            // 각 64비트 조각을 Shift하여 하나의 거대한 정수(N)로 합침
            pressure = pressure + (val << (i * 64));
        }
        pressure
    }
}

// [Brute-Force Point] 올바른 TimeSeed 찾기 검증 로직
fn check_alignment(seed: u32) -> bool {

    let salt = b"OrAcLe";
    let mut hasher = Sha256::new();
    hasher.update(seed.to_be_bytes());
    hasher.update(salt);
    let result = hasher.finalize();

    result[0] == 0 && result[1] == 0 && result[2] == 0
}

// Prefix 생성기 (TimeSeed에 의존)
fn generate_chronicle(seed: u32) -> Vec<u8> {
    let mut chronicle = Vec::new();
    let magic_constant = 0x1337_BEEF;
    
    // 88바이트의 Known Prefix 생성
    for i in 0..88 {
        // 선형 합동 생성기(LCG) 변형
        let val = (seed.wrapping_add(i as u32).wrapping_mul(magic_constant)) ^ 0xFF;
        chronicle.push((val >> 24) as u8);
    }
    chronicle
}

fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    // 입력값의 앞뒤 공백 및 개행문자(\n) 제거 후 바이트로 변환
    let flag_bytes = input_string.trim().as_bytes();

    // 1. [Brute-Force] 바이너리 내부에서 정답 Seed를 찾아서 암호화에 사용
    // 플레이어는 check_alignment 함수를 분석하여 이 값을 찾아내야 함
    let mut correct_seed = 0;
    
    // 0부터 순차적으로 대입하며 해시 조건(상위 3바이트 0)을 만족하는 seed 탐색
    for i in 0u32..u32::MAX {
        if check_alignment(i) {
            correct_seed = i;
            break;
        }
    }
    
    // 디버그용 출력 (문제 배포시에는 바이너리에서 문자열 제거됨)
    // println!("Debug: Seed found: {}", correct_seed); 

    // 2. [Crypto] RSA 파라미터 준비
    let n = ClockworkEngine::ignite_steam_engine();
    let e = BigInt::from(3u32);

    // 3. [Message] 평문 생성
    let mut plaintext = generate_chronicle(correct_seed);
    
    // [변경됨] 하드코딩된 값 대신 입력받은 flag_bytes 사용
    plaintext.extend_from_slice(flag_bytes);
    
    let m = BigInt::from_bytes_be(Sign::Plus, &plaintext);

    // 4. [Encrypt] 암호화
    let c = m.modpow(&e, &n);

    // 5. 출력
    println!("{}", c.to_str_radix(16));
}