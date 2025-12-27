fn main() {
        tonic_build::compile_protos("proto/secret.proto").unwrap();
}