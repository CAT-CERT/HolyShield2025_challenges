use sha2::{Sha256, Digest};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use rand::{seq::SliceRandom, SeedableRng, rngs::StdRng};
use tonic::{Request, Response, Status};
#[cfg(feature = "server")]
pub mod secret {
        tonic::include_proto!("hidden");
}
use crate::secret::{FlagRequest, FlagResponse, panic_verifier_server::PanicVerifier};
use crate::server::session;
use serde::Deserialize;
use std::process;
use hex;
use std::convert::TryInto; 

use super::secret_provider::{
        PUBLIC_IP, GRPC_TOKEN, GRPC_KEY, CHACHA_KEY, FINAL_PLAINTEXT, CHAM_KEY, 
            ULTIMATE_PLAINTEXT
};
pub use super::secret_provider::get_endpoint_and_key_hint;

pub const KEY_BYTES: &[u8] = b"ringover";
pub const ALLOWED_BASES: &[u32] = &[57, 58, 59, 61, 62];
pub const SUPER_ALPHABET: &str = "f8Z1qUshg6V2onE47Xk5RdpQeKyH9CGbWmLxMvaAjSNJTwzPBYrDc3tuFi0OIl";
pub const TARGET_CIPHER: &str =
    "R6GIDK4azALROzsFmFhj1fRTE4VJlKeCP83UpUW1icPCDcWgzp0cxR6t0NVWXJWL";

    fn prf32(key: &[u8], ctx: &[u8]) -> u32 {
            let mut hasher = Sha256::new();
                hasher.update(key);
                    hasher.update(ctx);
                        let result = hasher.finalize();
                            u32::from_be_bytes(result[0..4].try_into().expect("Hash slice error"))
    }
fn pos_base(pos_lsb: u32) -> u32 {
        let mut ctx = b"/base".to_vec();
            ctx.extend_from_slice(&pos_lsb.to_be_bytes());
                let hash_val = prf32(KEY_BYTES, &ctx);
                    let idx = hash_val % (ALLOWED_BASES.len() as u32);
                        ALLOWED_BASES[idx as usize]
}
fn pos_alphabet(pos_lsb: u32, base: u32) -> String {
        let mut ctx = b"/alpha".to_vec();
            ctx.extend_from_slice(&pos_lsb.to_be_bytes());
                let seed = prf32(KEY_BYTES, &ctx);
                    let mut rng = StdRng::seed_from_u64(seed as u64);
                        let mut bag: Vec<char> = SUPER_ALPHABET.chars().collect();
                            bag.shuffle(&mut rng);
                                bag.into_iter().take(base as usize).collect()
}
fn to_mixed_radix_digits(data: &[u8]) -> Vec<u32> {
        if data.is_empty() { return vec![0]; }
            let mut num = BigUint::from_bytes_be(data);
                if num.is_zero() { return vec![0]; }
                    let mut digits = Vec::new();
                        let mut pos = 0u32;
                            while num > BigUint::zero() {
                                        let base = pos_base(pos);
                                                let base_biguint = base.to_biguint().unwrap();
                                                        let remainder = (&num % &base_biguint).try_into().unwrap_or(0);
                                                                num /= base_biguint;
                                                                        digits.push(remainder);
                                                                                pos += 1;
                                                                                    }
                                digits
}
pub fn encode_mixed_radix(data: &[u8]) -> String {
        let digits = to_mixed_radix_digits(data);
            let n = digits.len();
                let mut out_chars = Vec::new();
                    for j in 0..n {
                                let pos_lsb = (n - 1 - j) as u32;
                                        let base = pos_base(pos_lsb);
                                                let alpha = pos_alphabet(pos_lsb, base);
                                                        let val = digits[pos_lsb as usize] as usize;
                                                                out_chars.push(alpha.chars().nth(val).unwrap()); 
                                                                    }
                        out_chars.into_iter().collect()
}
pub fn verify_input(input: &str) -> bool {
        encode_mixed_radix(input.as_bytes()) == TARGET_CIPHER
}
#[derive(Deserialize)]
struct InputData { Token: String, Key: String, }
pub fn secondary_check(input: &str) -> bool {
        let parsed: Result<InputData, _> = serde_json::from_str(input);
            if parsed.is_err() { process::exit(1); }
                let data = parsed.unwrap();
                    let token_match = data.Token == GRPC_TOKEN; 
                        let key_match = data.Key == GRPC_KEY;
                            let cipher_match = encode_mixed_radix(input.as_bytes()) == TARGET_CIPHER; 
                                token_match && key_match && cipher_match
}

pub mod chacha_logic {
        use std::convert::TryInto;

            #[no_mangle] pub static CONST: &[u8] = b"expand 32-byte k";
            #[no_mangle] pub static INITIAL_COUNTER: u32 = 1;

            #[no_mangle] #[inline(never)]
            pub extern "C" fn to_le_u32_words(b: &[u8]) -> Vec<u32> {
                        assert!(b.len() % 4 == 0, "bytes length must be a multiple of 4");
                                b.chunks_exact(4)
                                                .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
                                                            .collect()
                                                                }
                #[no_mangle] #[inline(never)]
                pub extern "C"  fn words_to_le_bytes(words: &[u32]) -> Vec<u8> {
                            words.iter().flat_map(|&w| w.to_le_bytes()).collect()
                                    }
                    #[no_mangle] #[inline(never)]
                    pub extern "C" fn quarter_round(a: u32, b: u32, c: u32, d: u32) -> (u32, u32, u32, u32) {
                                let mut a = a; let mut b = b; let mut c = c; let mut d = d;
                                        a = a.wrapping_add(b); d ^= a; d = d.rotate_left(16);
                                                c = c.wrapping_add(d); b ^= c; b = b.rotate_left(12);
                                                        a = a.wrapping_add(b); d ^= a; d = d.rotate_left(8);
                                                                c = c.wrapping_add(d); b ^= c; b = b.rotate_left(7);
                                                                        (a, b, c, d)
                                                                                }
                        #[no_mangle] #[inline(never)]
                        pub extern "C" fn qr_idx(st: &mut [u32; 16], i: usize, j: usize, k: usize, l: usize) {
                                    let (a, b, c, d) = quarter_round(st[i], st[j], st[k], st[l]);
                                            st[i] = a; st[j] = b; st[k] = c; st[l] = d;
                                                }
                            #[no_mangle] #[inline(never)]
                            pub extern "C" fn round_column(st: &mut [u32; 16]) {
                                        for s in 0..4 { qr_idx(st, s, s + 4, s + 8, s + 12); }
                                            }
                                #[no_mangle] #[inline(never)]
                                pub extern "C" fn round_diagonal(st: &mut [u32; 16]) {
                                            for i in 0..4 {
                                                            let a = i; let b = ((i + 1) & 3) + 4;
                                                                        let c = ((i + 2) & 3) + 8; let d = ((i + 3) & 3) + 12;
                                                                                    qr_idx(st, a, b, c, d);
                                                                                            }
                                                }
                                    #[no_mangle] #[inline(never)]
                                    pub extern "C" fn round_custom(st: &mut [u32; 16]) {
                                                qr_idx(st, 6, 3, 1, 0); qr_idx(st, 10, 7, 4, 2);
                                                        qr_idx(st, 13, 11, 8, 5); qr_idx(st, 15, 14, 12, 9);
                                                            }
                                        #[no_mangle] #[inline(never)]
                                        pub extern "C" fn build_state(key: &[u8], counter: u32, nonce: &[u8]) -> [u32; 16] {
                                                    if key.len() != 32 { panic!("key must be 32 bytes"); }
                                                            if nonce.len() != 12 { panic!("nonce must be 12 bytes"); }
                                                                    let const_words = to_le_u32_words(CONST);
                                                                            let key_words = to_le_u32_words(key);
                                                                                    let nonce_words = to_le_u32_words(nonce);
                                                                                            let mut state = [0u32; 16]; let mut i = 0;
                                                                                                    for w in const_words { state[i] = w; i += 1; }
                                                                                                            for w in key_words { state[i] = w; i += 1; }
                                                                                                                    state[i] = counter; i += 1;
                                                                                                                            for w in nonce_words { state[i] = w; i += 1; }
                                                                                                                                    state
                                                                                                                                            }
                                            #[no_mangle] #[inline(never)]
                                            pub extern "C" fn custom_chacha20_block(key: &[u8], counter: u32, nonce: &[u8]) -> Vec<u8> {
                                                        let x = build_state(key, counter, nonce); let mut st = x;
                                                                for _ in 0..10 {
                                                                                round_column(&mut st); round_diagonal(&mut st); round_custom(&mut st);
                                                                                        }
                                                                        for i in 0..16 { st[i] = st[i].wrapping_add(x[i]); }
                                                                                words_to_le_bytes(&st)
                                                                                        }
                                                #[no_mangle] #[inline(never)]
                                                pub extern "C" fn keystream(key: &[u8], nonce: &[u8], counter: u32, nbytes: usize) -> Vec<u8> {
                                                            let mut out = Vec::with_capacity(nbytes); let mut ctr = counter;
                                                                    while out.len() < nbytes {
                                                                                    let block = custom_chacha20_block(key, ctr, nonce);
                                                                                                let take = (nbytes - out.len()).min(64);
                                                                                                            out.extend_from_slice(&block[..take]);
                                                                                                                        ctr = ctr.wrapping_add(1);
                                                                                                                                }
                                                                            out
                                                                                    }
                                                    #[no_mangle] #[inline(never)]
                                                    pub extern "C" fn encrypt(key: &[u8], nonce: &[u8], counter: u32, plaintext: &[u8]) -> Vec<u8> {
                                                                let ks = keystream(key, nonce, counter, plaintext.len());
                                                                        plaintext.iter().zip(ks.iter()).map(|(p, k)| p ^ k).collect()
                                                                                }
}

pub mod cham_logic {
        use std::convert::TryInto;
            #[no_mangle] #[inline(never)]
            pub extern "C" fn to_u32_le_words(b: &[u8]) -> Vec<u32> {
                        assert!(b.len() % 4 == 0, "Input bytes must be a multiple of 4");
                                b.chunks_exact(4)
                                                .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
                                                            .collect()
                                                                }
                #[no_mangle] #[inline(never)]
                pub extern "C" fn from_u32_le_words(ws: &[u32]) -> Vec<u8> {
                            ws.iter().flat_map(|w| w.to_le_bytes()).collect()
                                    }
                    #[no_mangle] #[inline(never)]
                    pub extern "C" fn cham_key_schedule_128_256(key32: &[u8]) -> (Vec<u32>, Vec<u32>) {
                                let k = to_u32_le_words(key32);
                                        let rk_even: Vec<u32> = k
                                                        .iter()
                                                                    .map(|&ki| ki ^ ki.rotate_left(1) ^ ki.rotate_left(8))
                                                                                .collect();
                                                let rk_odd: Vec<u32> = k
                                                                .iter()
                                                                            .map(|&ki| ki ^ ki.rotate_left(1) ^ ki.rotate_left(11))
                                                                                        .collect();
                                                        (rk_even, rk_odd)
                                                                }
                        #[no_mangle] #[inline(never)]
                        pub extern "C" fn cham128_256_encrypt_block(
                                    key32: &[u8], block16: &[u8], rounds: usize,
                                        ) -> [u8; 16] {
                                    let (rk_even, rk_odd) = cham_key_schedule_128_256(key32);
                                            let words = to_u32_le_words(block16);
                                                    let (mut s0, mut s1, mut s2, mut s3) = (words[0], words[1], words[2], words[3]);
                                                            for i in 0..rounds {
                                                                            let i_u32 = i as u32;
                                                                                        let new_s;
                                                                                                    if (i & 1) == 0 {
                                                                                                                        let t = s1.rotate_left(1) ^ rk_even[i % 8];
                                                                                                                                        new_s = (s0 ^ i_u32).wrapping_add(t).rotate_left(8);
                                                                                                                                                    } else {
                                                                                                                                                                        let t = s1.rotate_left(8) ^ rk_odd[i % 8];
                                                                                                                                                                                        new_s = (s0 ^ i_u32).wrapping_add(t).rotate_left(1);
                                                                                                                                                                                                    }
                                                                                                                s0 = s1; s1 = s2; s2 = s3; s3 = new_s;
                                                                                                                        }
                                                                    let result_vec = from_u32_le_words(&[s3, s2, s1, s0]);
                                                                            result_vec.try_into().expect("Block size must be 16 bytes")
                                                                                    }
                            #[inline(never)]
                            pub extern "C" fn encrypt(
                                        key32: &[u8], nonce16: &[u8], plaintext: &[u8],
                                            ) -> Result<Vec<u8>, &'static str> {
                                        if key32.len() != 32 { return Err("Key must be 32 bytes"); }
                                                if nonce16.len() != 16 { return Err("Nonce must be 16 bytes"); }
                                                        let n12 = &nonce16[0..12];
                                                                let mut out = Vec::new();
                                                                        for (i_zero_based, p_chunk) in plaintext.chunks(16).enumerate() {
                                                                                        let i = i_zero_based + 1;
                                                                                                    let i_be_bytes = (i as u32).to_be_bytes();
                                                                                                                let mut cb = [0u8; 16];
                                                                                                                            cb[0..12].copy_from_slice(n12);
                                                                                                                                        cb[12..16].copy_from_slice(&i_be_bytes);
                                                                                                                                                    let s = cham128_256_encrypt_block(key32, &cb, 120);
                                                                                                                                                                let s_truncated = &s[..p_chunk.len()];
                                                                                                                                                                            let c_chunk: Vec<u8> = p_chunk
                                                                                                                                                                                                .iter()
                                                                                                                                                                                                                .zip(s_truncated.iter())
                                                                                                                                                                                                                                .map(|(&p_byte, &s_byte)| p_byte ^ s_byte)
                                                                                                                                                                                                                                                .collect();
                                                                                                                                                                                        out.extend(c_chunk);
                                                                                                                                                                                                }
                                                                                Ok(out)
                                                                                        }
}


fn verify_grpc_pair(token: &str, key: &str) -> bool {
        token == GRPC_TOKEN && key == GRPC_KEY
}
#[derive(Default)]
pub struct FlagSvc;
#[tonic::async_trait]
impl PanicVerifier for FlagSvc {
        async fn flag(
                    &self, req: Request<FlagRequest>,
                        ) -> Result<Response<FlagResponse>, Status> {
                    let msg = req.into_inner();
                            if !verify_grpc_pair(&msg.token, &msg.key) {
                                            return Err(Status::unauthenticated("invalid token/key"));
                                                    }
                                    let id = msg.challenge_id.trim();
                                            if id.is_empty() {
                                                            return Err(Status::invalid_argument("id must not be empty"));
                                                                    }
                                                    if !session::has_challenge(id) {
                                                                    return Err(Status::permission_denied("unknown challenge id"));
                                                                            }
                                                            let sess_hex_12 = session::generate_session(id);
                                                                    let nonce_bytes_12 = match hex::decode(&sess_hex_12) {
                                                                                    Ok(bytes) if bytes.len() == 12 => bytes,
                                                                                                _ => return Err(Status::internal("Nonce generation failed")),
                                                                                                        };
                                                                            let chacha_key = CHACHA_KEY.as_bytes();
                                                                                    let chacha_counter = self::chacha_logic::INITIAL_COUNTER;
                                                                                            let chacha_plaintext_bytes = FINAL_PLAINTEXT.as_bytes();
                                                                                                    let chacha_cipher_bytes = self::chacha_logic::encrypt( 
                                                                                                                    chacha_key, &nonce_bytes_12, chacha_counter, chacha_plaintext_bytes,
                                                                                                                            );
                                                                                                            let chacha_cipher_hex = hex::encode(chacha_cipher_bytes);
                                                                                                                    session::store_last_cipher(id, &chacha_cipher_hex);
                                                                                                                            session::set_auth_success(id);
                                                                                                                                    let reply = FlagResponse {
                                                                                                                                                    session: sess_hex_12, message: chacha_cipher_hex,
                                                                                                                                                            };
                                                                                                                                            Ok(Response::new(reply))
                                                                                                                                                    }
}

pub fn verify_chacha_input(id: &str, user_input: &str) -> (bool, String) {
        let sess_hex_12 = match session::get_last_session(id) {
                    Some(s) => s, None => return (false, "NO_SESSION".to_string()),
                        };
            let nonce_bytes_12 = match hex::decode(&sess_hex_12) {
                        Ok(bytes) if bytes.len() == 12 => bytes,
                                _ => return (false, "NONCE_ERROR".to_string()),
                                    };
                let key = CHACHA_KEY.as_bytes();
                    let counter = self::chacha_logic::INITIAL_COUNTER;
                        let user_encrypted_bytes =
                                    self::chacha_logic::encrypt(key, &nonce_bytes_12, counter, user_input.as_bytes());
                            let user_cipher_hex = hex::encode(user_encrypted_bytes);
                                let correct_cipher_hex = match session::get_last_cipher(id) {
                                            Some(c) => c, None => "SERVER_ERROR".to_string(),
                                                };
                                    let is_match = user_cipher_hex == correct_cipher_hex;
                                        (is_match, user_cipher_hex)
}
pub fn generate_and_get_cham_cipher(id: &str, cham_nonce_hex_16: &str) -> Option<String> {
        let nonce_bytes_16 = match hex::decode(cham_nonce_hex_16) {
                    Ok(bytes) if bytes.len() == 16 => bytes, _ => return None,
                        };
            let cham_plaintext_bytes = ULTIMATE_PLAINTEXT.as_bytes();
                match self::cham_logic::encrypt(CHAM_KEY, &nonce_bytes_16, cham_plaintext_bytes) {
                            Ok(bytes) => {
                                            let cipher_hex = hex::encode(bytes);
                                                        session::store_last_cham_cipher(id, &cipher_hex);
                                                                    Some(cipher_hex)
                                                                                }
                                    Err(_) => None,
                                        }
}
pub fn verify_cham_input(id: &str, user_input: &str, cham_nonce_hex_16: &str) -> (bool, String) {
        let correct_cipher_hex = match session::get_last_cham_cipher(id) {
                    Some(c) => c, None => return (false, "SERVER_ERROR".to_string()),
                        };
            let nonce_bytes_16 = match hex::decode(cham_nonce_hex_16) {
                        Ok(bytes) if bytes.len() == 16 => bytes, _ => return (false, "NONCE_ERROR".to_string()),
                            };
                let user_cipher_hex = match self::cham_logic::encrypt(CHAM_KEY, &nonce_bytes_16, user_input.as_bytes()) {
                            Ok(bytes) => hex::encode(bytes),
                                    Err(_) => "CHAM_ENCRYPT_ERROR".to_string(),
                                        };
                    let is_match = user_cipher_hex == correct_cipher_hex;
                        (is_match, user_cipher_hex)
}
