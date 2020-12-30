fn main() {
    tonic_build::compile_protos("src/proto/raft.proto").unwrap();
}