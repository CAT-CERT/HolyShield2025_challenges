use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ChallengeState {
            pub session: Option<String>,
                        pub chacha_cipher: Option<String>,
                                        pub cham_cipher: Option<String>,
                                                            pub auth_success: bool,
}

static CHALLENGES: Lazy<Mutex<HashMap<String, ChallengeState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

        pub fn init_challenge(id: &str) {
                        let mut map = CHALLENGES.lock().unwrap();
                                        map.insert(
                                                                        id.to_string(),
                                                                                                            ChallengeState {
                                                                                                                                                                    session: None,
                                                                                                                                                                                                                                    chacha_cipher: None,
                                                                                                                                                                                                                                                                                                                cham_cipher: None,
                                                                                                                                                                                                                                                                                                                                                                                                        auth_success: false,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        },
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            );
                                            }

pub fn generate_session(id: &str) -> String {
            let mut bytes = [0u8; 12];
                        rand::thread_rng().fill(&mut bytes);
                                        let sess = hex::encode(bytes);

                                                            let mut map = CHALLENGES.lock().unwrap();
                                                                                    if let Some(st) = map.get_mut(id) {
                                                                                                                            st.session = Some(sess.clone());
                                                                                                                                                                    }

                                                                                                                sess
}

pub fn get_last_session(id: &str) -> Option<String> {
            let map = CHALLENGES.lock().unwrap();
                        map.get(id).and_then(|st| st.session.clone())
}

pub fn set_auth_success(id: &str) {
            let mut map = CHALLENGES.lock().unwrap();
                        if let Some(st) = map.get_mut(id) {
                                                    st.auth_success = true;
                                                                                }
}

pub fn is_auth_success(id: &str) -> bool {
            let map = CHALLENGES.lock().unwrap();
                        map.get(id).map(|st| st.auth_success).unwrap_or(false)
}

pub fn store_last_cipher(id: &str, cipher: &str) {
            let mut map = CHALLENGES.lock().unwrap();
                        if let Some(st) = map.get_mut(id) {
                                                    st.chacha_cipher = Some(cipher.to_string());
                                                                                }
}

pub fn get_last_cipher(id: &str) -> Option<String> {
            let map = CHALLENGES.lock().unwrap();
                        map.get(id).and_then(|st| st.chacha_cipher.clone())
}

pub fn store_last_cham_cipher(id: &str, cipher: &str) {
            let mut map = CHALLENGES.lock().unwrap();
                        if let Some(st) = map.get_mut(id) {
                                                    st.cham_cipher = Some(cipher.to_string());
                                                                                }
}

pub fn get_last_cham_cipher(id: &str) -> Option<String> {
            let map = CHALLENGES.lock().unwrap();
                        map.get(id).and_then(|st| st.cham_cipher.clone())
}

pub fn clear_challenge(id: &str) {
            let mut map = CHALLENGES.lock().unwrap();
                        map.remove(id);
}

pub fn has_challenge(id: &str) -> bool {
            let map = CHALLENGES.lock().unwrap();
                        map.contains_key(id)
}

