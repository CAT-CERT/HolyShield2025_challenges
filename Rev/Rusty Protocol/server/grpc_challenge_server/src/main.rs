#[used]
#[no_mangle]
#[link_section = ".rodata.panic"]
pub static SECRET_PROTO_SRC: &str = r#"syntax = "proto3";
package hidden;

service PanicVerifier {
        rpc Flag (FlagRequest) returns (FlagResponse);
        }

        message FlagRequest {
            string token      = 1;
                string key        = 2;
                    string challenge_id = 3;
                    }

                    message FlagResponse {
                        string session = 1;
                            string message = 2;
                            }
                            "#;
                            #[no_mangle]
                            pub fn leak_proto_src() -> &'static str {
                                    SECRET_PROTO_SRC
                            }

mod server;
use server::{verify, session}; 

use tonic::transport::Server;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};
#[cfg(feature = "server")]
use self::secret::panic_verifier_server::PanicVerifierServer;
#[cfg(feature = "server")]
pub mod secret {
        tonic::include_proto!("hidden");
}

#[no_mangle] pub static KEY_BYTES: &[u8] = b"ringover";
#[no_mangle] pub static ALLOWED_BASES: &[u32] = &[57, 58, 59, 61, 62];
#[no_mangle] pub static SUPER_ALPHABET: &str = "f8Z1qUshg6V2onE47Xk5RdpQeKyH9CGbWmLxMvaAjSNJTwzPBYrDc3tuFi0OIl";
#[no_mangle] pub static TARGET_CIPHER: &str = "R6GIDK4azALROzsFmFhj1fRTE4VJlKeCP83UpUW1icPCDcWgzp0cxR6t0NVWXJWL";

async fn handle_connection(stream: TcpStream) {
        if let Ok(peer) = stream.peer_addr() {
                    println!("[+] New connection from {}", peer);
                        }

            let (reader, mut writer) = stream.into_split();
                let mut reader = BufReader::new(reader);
                    let mut line = String::new();

                        writer.write_all(b"Enter your challenge_id:\n")
                                    .await.ok();
                            writer.flush().await.ok();

                                line.clear();
                                    if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                                                return;
                                                    }
                                        let challenge_id = line.trim().to_string();
                                            if challenge_id.is_empty() {
                                                        return;
                                                            }

                                                session::init_challenge(&challenge_id);

                                                    line.clear();
                                                        if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                                                                    return;
                                                                        }
                                                            let input = line.trim();

                                                                if !verify::verify_input(input) { 
                                                                            return;
                                                                                }
                                                                    if !verify::secondary_check(input) { 
                                                                                return;
                                                                                    }

                                                                        let hint = verify::get_endpoint_and_key_hint(&challenge_id);
                                                                            let _ = writer.write_all(hint.as_bytes()).await;
                                                                                let _ = writer.flush().await;

                                                                                    let mut waited = 0u64;
                                                                                        while !session::is_auth_success(&challenge_id) {
                                                                                                    if waited > 5 {
                                                                                                                    session::clear_challenge(&challenge_id);
                                                                                                                                return;
                                                                                                                                        }
                                                                                                            tokio::time::sleep(Duration::from_secs(1)).await;
                                                                                                                    waited += 1;
                                                                                                                        }


                                                                                            let sess_hex_12 = match session::get_last_session(&challenge_id) {
                                                                                                        Some(s) => s,
                                                                                                                None => {
                                                                                                                                let _ = writer.write_all(b"NO_SESSION.\n").await;
                                                                                                                                            let _ = writer.flush().await;
                                                                                                                                                        session::clear_challenge(&challenge_id);
                                                                                                                                                                    return;
                                                                                                                                                                            }
                                                                                                            };

                                                                                                let correct_chacha_cipher_hex = match session::get_last_cipher(&challenge_id) {
                                                                                                            Some(c) => c,
                                                                                                                    None => {
                                                                                                                                    let _ = writer.write_all(b"SERVER_ERROR.\n").await;
                                                                                                                                                let _ = writer.flush().await;
                                                                                                                                                            session::clear_challenge(&challenge_id);
                                                                                                                                                                        return;
                                                                                                                                                                                }
                                                                                                                };


                                                                                                    line.clear();
                                                                                                        let chacha_res = timeout(Duration::from_secs(10), reader.read_line(&mut line)).await;

                                                                                                            match chacha_res {
                                                                                                                        Ok(Ok(0)) | Ok(Err(_)) => {
                                                                                                                                        session::clear_challenge(&challenge_id);
                                                                                                                                                    return;
                                                                                                                                                            }
                                                                                                                                Err(_) => {
                                                                                                                                                session::clear_challenge(&challenge_id);
                                                                                                                                                            return;
                                                                                                                                                                    }
                                                                                                                                        Ok(Ok(_)) => {}
                                                                                                                                            }

                                                                                                                let chacha_plaintext_input = line.trim();
                                                                                                                    
                                                                                                                    let (is_match, user_chacha_cipher_hex) =
                                                                                                                                verify::verify_chacha_input(&challenge_id, chacha_plaintext_input);

                                                                                                                        let server_match_chacha = is_match;
                                                                                                                            let main_match_chacha = user_chacha_cipher_hex == correct_chacha_cipher_hex; 

                                                                                                                                if !(is_match) { 
                                                                                                                                            session::clear_challenge(&challenge_id);
                                                                                                                                                    return;
                                                                                                                                                        }

                                                                                                                                    let cham_nonce_hex_16 = format!("{}00000000", sess_hex_12);

                                                                                                                                        let correct_cham_cipher_hex =
                                                                                                                                                    match verify::generate_and_get_cham_cipher(&challenge_id, &cham_nonce_hex_16) {
                                                                                                                                                                    Some(c) => c,
                                                                                                                                                                                None => {
                                                                                                                                                                                                    session::clear_challenge(&challenge_id);
                                                                                                                                                                                                                    return;
                                                                                                                                                                                                                                }
                                                                                                                                                                            };


                                                                                                                                            line.clear();
                                                                                                                                                let cham_res = timeout(Duration::from_secs(10), reader.read_line(&mut line)).await;

                                                                                                                                                    match cham_res {
                                                                                                                                                                Ok(Ok(0)) | Ok(Err(_)) => {
                                                                                                                                                                                session::clear_challenge(&challenge_id);
                                                                                                                                                                                            return;
                                                                                                                                                                                                    }
                                                                                                                                                                        Err(_) => {
                                                                                                                                                                                        session::clear_challenge(&challenge_id);
                                                                                                                                                                                                    return;
                                                                                                                                                                                                            }
                                                                                                                                                                                Ok(Ok(_)) => {}
                                                                                                                                                                                    }

                                                                                                                                                        let final_plaintext_input = line.trim();
                                                                                                                                                            
                                                                                                                                                            let (is_match, user_cham_cipher_hex) =
                                                                                                                                                                        verify::verify_cham_input(&challenge_id, final_plaintext_input, &cham_nonce_hex_16);

                                                                                                                                                                let correct_cham_cipher_hex_main =
                                                                                                                                                                            match session::get_last_cham_cipher(&challenge_id) {
                                                                                                                                                                                            Some(c) => c,
                                                                                                                                                                                                        None => {
                                                                                                                                                                                                                            session::clear_challenge(&challenge_id);
                                                                                                                                                                                                                                            return;
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                    };

                                                                                                                                                                    let server_match_cham = is_match;
                                                                                                                                                                        let main_match_cham = user_cham_cipher_hex == correct_cham_cipher_hex_main;

                                                                                                                                                                            if is_match {
                                                                                                                                                                                        let _ = writer
                                                                                                                                                                                                        .write_all(b"\nCONGRATULATIONS! AUTH SUCCESS\n")
                                                                                                                                                                                                                    .await;
                                                                                                                                                                                                let _ = writer.flush().await;
                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                session::clear_challenge(&challenge_id);
                                                                                                                                                                                                                        return;
                                                                                                                                                                                                                            }

                                                                                                                                                                                session::clear_challenge(&challenge_id);
}


use server::verify::FlagSvc;
#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let grpc_addr = "0.0.0.0:5000".parse()?;
            tokio::spawn(async move {
                        match Server::builder()
                                        .add_service(PanicVerifierServer::new(FlagSvc::default()))
                                                    .serve(grpc_addr)
                                                                .await
                                                                            {
                                                                                                Ok(_) => println!("gRPC server started on 5000"),
                                                                                                                Err(e) => eprintln!("gRPC server error: {}", e),
                                                                                                                            }
                            });

                let challenge_addr = "0.0.0.0:5001";
                    let listener = TcpListener::bind(challenge_addr).await?;
                        println!("Challenge server running on 5001");

                            loop {
                                        let (stream, _) = listener.accept().await?;
                                                tokio::spawn(async move {
                                                                handle_connection(stream).await;
                                                                        });
                                                    }
}
