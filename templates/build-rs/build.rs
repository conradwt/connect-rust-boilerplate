fn main() {
    connectrpc_build::Config::new()
        .include_file("_connectrpc.rs")
        .files(&["proto/{{proto_name}}.proto"])
        .includes(&["proto"])
        .compile()
        .expect("failed to compile proto files");
}
