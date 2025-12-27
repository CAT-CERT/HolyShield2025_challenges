pub const PUBLIC_IP: &str = "34.64.228.73";
pub const CHACHA_KEY: &str = "j_k0o'g_oNnk_Gr_k3hAvUg_7w!D_0o3";

pub const GRPC_TOKEN: &str = "HolyShield2025";
pub const GRPC_KEY: &str = "ringpanic";
pub const FINAL_PLAINTEXT: &str = "i_7H1nk_U'v3_DoNe_QUiTe_@_LO7_:)";
pub const CHAM_KEY: &[u8; 32] = b"i_7H1nk_U'v3_DoNe_QUiTe_@_LO7_:)";
pub const ULTIMATE_PLAINTEXT: &str = "HolyShield{i_w45_p4N1cKED_bY_dECrypti0n!!}";


pub const KEY_BYTES: &[u8] = b"ringover";
pub const ALLOWED_BASES: &[u32] = &[57, 58, 59, 61, 62];
pub const SUPER_ALPHABET: &str = "f8Z1qUshg6V2onE47Xk5RdpQeKyH9CGbWmLxMvaAjSNJTwzPBYrDc3tuFi0OIl";

pub fn get_endpoint_and_key_hint(_id: &str) -> String {
        format!(
                    "Target Address: {}:5000\n{{hint: hidden.PanicVerifier/Flag}}\n{{key:{}}}\n",
                            PUBLIC_IP,
                                    CHACHA_KEY,
                                        )
}
