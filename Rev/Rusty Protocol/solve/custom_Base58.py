use sha2::{Sha256, Digest};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use rand::{seq::SliceRandom, SeedableRng, rngs::StdRng};

const CIPHER_TEXT: &str = "O3WDSeUgjSyuoS0pbsZn25hGvl3eDaYczdidqgFOZQOCV0I5ryiz1";
const KEY_BYTES: &[u8] = b"ringover";
const ALLOWED_BASES: &[u32] = &[57, 58, 59, 61, 62];
const SUPER_ALPHABET: &str = "f8Z1qUshg6V2onE47Xk5RdpQeKyH9CGbWmLxMvaAjSNJTwzPBYrDc3tuFi0OIl";

fn prf32(key: &[u8], ctx: &[u8]) -> u32 {
        let mut hasher = Sha256::new();
            hasher.update(key);
                hasher.update(ctx);
                    let result = hasher.finalize();
                        let slice: [u8; 4] = result[0..4].try_into().expect("Slice with incorrect length");
                            u32::from_be_bytes(slice)
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

fn digits_from_mixed_radix_chars(s: &str) -> Result<Vec<u32>, &'static str> {
        let n = s.len();
            let mut digits_lsb = vec![0u32; n];
                for (j, ch) in s.chars().enumerate() {
                            let pos_lsb = (n - 1 - j) as u32;
                                    let base = pos_base(pos_lsb);
                                            let alpha = pos_alphabet(pos_lsb, base);
                                                    
                                                    if let Some(v) = alpha.find(ch) {
                                                                    digits_lsb[pos_lsb as usize] = v as u32;
                                                                            } else {
                                                                                            return Err("Character not found in alphabet");
                                                                                                    }
                                                        }
                    Ok(digits_lsb)
}

fn from_mixed_radix_digits(digits_lsb: &[u32]) -> BigUint {
        let n = digits_lsb.len();
            if n == 0 {
                        return BigUint::zero();
                            }
                
                let mut num = digits_lsb[n - 1].to_biguint().unwrap(); 

                    for p in (0..=(n - 2)).rev() {
                                let base = pos_base(p as u32).to_biguint().unwrap();
                                        let digit = digits_lsb[p].to_biguint().unwrap();
                                                
                                                num = num * base + digit;
                                                    }
                        num
}

fn decode_mixed_radix(s: &str) -> Result<Vec<u8>, &'static str> {
        let digits = digits_from_mixed_radix_chars(s)?;
            
            if digits.is_empty() {
                        return Ok(Vec::new());
                            }
                
                let num = from_mixed_radix_digits(&digits);

                    if num.is_zero() {
                                return Ok(b"\x00".to_vec());
                                    }

                        Ok(num.to_bytes_be())
}

fn main() {
        println!("[DECRYPT RESULT] (Using Rust's native StdRng)");

            let key_hex = KEY_BYTES.iter()
                                           .map(|b| format!("{:02x}", b))
                                                                      .collect::<String>();
                println!("KEY(hex)   : {}", key_hex);

                    match decode_mixed_radix(CIPHER_TEXT) {
                                Ok(recovered) => {
                                                match String::from_utf8(recovered) {
                                                                    Ok(s) => {
                                                                                            println!("PLAINTEXT  : {}", s);
                                                                                                            }
                                                                                    Err(e) => {
                                                                                                            println!("(PLAINTEXT is not valid UTF-8)");
                                                                                                                                println!("PLAINTEXT(hex): {}", 
                                                                                                                                                                 e.into_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>()
                                                                                                                                                                                     );
                                                                                                                                                }
                                                                                                }
                                                        }
                                        Err(e) => {
                                                        println!("DECODING FAILED: {}", e);
                                                                }
                                            }
                        
                        println!("\n(NOTE: Rust's RNG is different from Python's, so the decoded result is expected to be meaningless bytes.)");
}
