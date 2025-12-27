// src/minigames/check_keys/key_derivation.rs
// 키와 Nonce를 여러 단계를 거쳐 유도

use super::constants::*;

// 1단계: 기본 키 조립
fn assemble_raw_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    
    key[0..8].copy_from_slice(&SHARD_ALPHA);
    key[8..16].copy_from_slice(&SHARD_BETA);
    key[16..24].copy_from_slice(&SHARD_GAMMA);
    key[24..32].copy_from_slice(&SHARD_DELTA);
    
    key
}

// 2단계: 키 변환 (비선형 변환)
fn transform_key_stage1(key: &mut [u8; 32]) {
    for i in 0..32 {
        let idx = (i + 7) % 32;
        key[i] ^= key[idx];
    }
}

// 3단계: 키 섞기
fn transform_key_stage2(key: &mut [u8; 32]) {
    for i in (0..16).rev() {
        key.swap(i, 31 - i);
    }
}

// 4단계: 키 역변환
fn transform_key_stage3(key: &mut [u8; 32]) {
    for i in 0..32 {
        key[i] = key[i].wrapping_add(i as u8);
    }
}

// 최종 키 유도
pub fn derive_master_key() -> [u8; 32] {
    let mut key = assemble_raw_key();
    
    transform_key_stage1(&mut key);
    transform_key_stage2(&mut key);
    transform_key_stage3(&mut key);
    
    key
}

// Nonce 조립 (간접 참조)
fn build_raw_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    
    nonce[..6].copy_from_slice(&IV_FRONT);
    nonce[6..].copy_from_slice(&IV_BACK);
    
    nonce
}

// Nonce 변환
fn transform_nonce(nonce: &mut [u8; 12]) {
    for i in 0..12 {
        nonce[i] = nonce[i].wrapping_mul(3).wrapping_add(7);
    }
}

pub fn derive_nonce() -> [u8; 12] {
    let mut nonce = build_raw_nonce();
    transform_nonce(&mut nonce);
    nonce
}

// 역변환 함수들 (복호화용)
pub fn reverse_key_derivation(key: &mut [u8; 32]) {
    // Stage 3 역변환
    for i in 0..32 {
        key[i] = key[i].wrapping_sub(i as u8);
    }
    
    // Stage 2 역변환
    for i in (0..16).rev() {
        key.swap(i, 31 - i);
    }
    
    // Stage 1 역변환
    for i in (0..32).rev() {
        let idx = (i + 7) % 32;
        key[i] ^= key[idx];
    }
}

pub fn reverse_nonce_derivation(nonce: &mut [u8; 12]) {
    for i in 0..12 {
        // (x * 3 + 7)의 역연산
        // modular inverse of 3 mod 256 is 171
        nonce[i] = nonce[i].wrapping_sub(7).wrapping_mul(171);
    }
}

// 키 검증용 체크섬
pub fn compute_key_checksum(key: &[u8; 32]) -> u32 {
    key.iter().fold(0u32, |acc, &b| acc.wrapping_add(b as u32))
}

// 참조 포인터 체인 (난독화)
pub fn get_key_ptr() -> *const u8 {
    SHARD_ALPHA.as_ptr()
}

pub fn get_nonce_ptr() -> *const u8 {
    IV_FRONT.as_ptr()
}

// 간접 로딩 함수
pub fn load_encrypted_payload() -> Vec<u8> {
    PAYLOAD.to_vec()
}