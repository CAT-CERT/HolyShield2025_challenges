// src/minigames/check_keys/validation.rs
// 검증 로직 (해시 기반) - XOR layer 제거 버전

use super::constants::ANSWER_HASH;
use super::crypto_core::apply_keystream;
use super::key_derivation::{derive_master_key, derive_nonce, load_encrypted_payload};

// SHA256 hash calculation (simple)
fn compute_sha256_hash(input: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}

fn encrypt_normalized_input(normalized_input: &str) -> Vec<u8> {
    let mut data = normalized_input.as_bytes().to_vec();
    let key = derive_master_key();
    let nonce = derive_nonce();

    // ChaCha20만 적용 (XOR layer 제거)
    apply_keystream(&mut data, &key, &nonce);

    data
}

// Input verification
pub fn verify_flag_input(user_input: &str) -> bool {
    let normalized = user_input.to_lowercase().trim().to_string();
    let encrypted = encrypt_normalized_input(&normalized);
    let hash = compute_sha256_hash(&encrypted);

    hash == ANSWER_HASH
}


// 복호화 함수 (디버깅용 - 실제론 사용 안 함)
#[allow(dead_code)]
pub fn decrypt_flag_internal() -> Result<String, String> {
    let mut payload = load_encrypted_payload();
    
    // ChaCha20 복호화만 수행 (XOR layer 제거)
    let key = derive_master_key();
    let nonce = derive_nonce();
    
    apply_keystream(&mut payload, &key, &nonce);
    
    // UTF-8 변환
    String::from_utf8(payload).map_err(|_| "Decryption failed".to_string())
}

// 체크섬 검증 (추가 보안)
pub fn verify_integrity() -> bool {
    let key = derive_master_key();
    let checksum = super::key_derivation::compute_key_checksum(&key);
    
    // 예상 체크섬 (키 변환 후)
    checksum != 0
}

// 힌트 생성
pub fn generate_hint() -> String {
    let payload = load_encrypted_payload();
    format!("Encrypted ({} bytes): {:02x?}...", payload.len(), &payload[..16])
}

// 다중 체크 (타이밍 공격 방어)
pub fn secure_verify(input: &str) -> bool {
    use std::time::{Duration, Instant};
    
    let start = Instant::now();
    let result = verify_flag_input(input);
    
    // 최소 100ms 대기 (타이밍 공격 방어)
    let elapsed = start.elapsed();
    if elapsed < Duration::from_millis(100) {
        std::thread::sleep(Duration::from_millis(100) - elapsed);
    }
    
    result
}
