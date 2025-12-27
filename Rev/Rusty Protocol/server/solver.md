grpc_challenge_server/
├── Cargo.toml
├── build.rs
├── proto/
│   └── secret.proto
├── src/
│   ├── main.rs
│   ├── server
│   │   ├── verify.rs            # 정답 인증 + 엔드포인트 출력 + session.rs 호출
│   │   ├── session.rs           # 세션 생성 로직
│   │   ├── secret_provider.rs   # 핵심 키/평문/상수 저장
│   │   └── mod.rs