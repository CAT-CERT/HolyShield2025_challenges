// src/minigames/check_keys/crypto_core.rs
// 직역: ChaCha20 블록 함수 (다크한 암호화 파이프라인)

use super::constants::{MAGIC_1, MAGIC_2, MAGIC_3, MAGIC_4, XOR_MASK_1, XOR_MASK_2};

const ROUNDS: usize = 20;

#[inline(always)]
fn rotl32(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

#[inline(always)]
fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = rotl32(state[d], 16);

    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = rotl32(state[b], 12);

    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = rotl32(state[d], 8);

    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = rotl32(state[b], 7);
}

pub fn chacha20_block(key: &[u8; 32], nonce: &[u8; 12], counter: u32) -> [u8; 64] {
    let mut state = [0u32; 16];

    state[0] = MAGIC_1;
    state[1] = MAGIC_2;
    state[2] = MAGIC_3;
    state[3] = MAGIC_4;

    for i in 0..8 {
        state[4 + i] = u32::from_le_bytes([
            key[i * 4],
            key[i * 4 + 1],
            key[i * 4 + 2],
            key[i * 4 + 3],
        ]);
    }

    state[12] = counter;

    for i in 0..3 {
        state[13 + i] = u32::from_le_bytes([
            nonce[i * 4],
            nonce[i * 4 + 1],
            nonce[i * 4 + 2],
            nonce[i * 4 + 3],
        ]);
    }

    let mut working_state = state;

    for _ in 0..(ROUNDS / 2) {
        quarter_round(&mut working_state, 0, 4, 8, 12);
        quarter_round(&mut working_state, 1, 5, 9, 13);
        quarter_round(&mut working_state, 2, 6, 10, 14);
        quarter_round(&mut working_state, 3, 7, 11, 15);

        quarter_round(&mut working_state, 0, 5, 10, 15);
        quarter_round(&mut working_state, 1, 6, 11, 12);
        quarter_round(&mut working_state, 2, 7, 8, 13);
        quarter_round(&mut working_state, 3, 4, 9, 14);
    }

    for i in 0..16 {
        working_state[i] = working_state[i].wrapping_add(state[i]);
    }

    let mut output = [0u8; 64];
    for i in 0..16 {
        let bytes = working_state[i].to_le_bytes();
        output[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
    }

    output
}

pub fn apply_keystream(data: &mut [u8], key: &[u8; 32], nonce: &[u8; 12]) {
    let mut counter = 0u32;
    let mut offset = 0;

    while offset < data.len() {
        let keystream = chacha20_block(key, nonce, counter);

        let chunk_len = (data.len() - offset).min(64);
        for i in 0..chunk_len {
            data[offset + i] ^= keystream[i];
        }

        counter = counter.wrapping_add(1);
        offset += chunk_len;
    }
}

pub fn apply_xor_layer(data: &mut [u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        if i % 2 == 0 {
            *byte ^= XOR_MASK_1;
        } else {
            *byte ^= XOR_MASK_2;
        }
    }
}

pub fn remove_xor_layer(data: &mut [u8]) {
    apply_xor_layer(data);
}
