# Connect Rust - Checked In Code Template (Buf Generate)

This template scaffolds a ConnectRPC service that uses the `buf` toolchain to compile Protocol Buffer definitions into checked-in Rust files.

## Tech Stack
- **Language**: Rust (Edition 2024)
- **Protocol**: ConnectRPC, gRPC, and gRPC-Web
- **HTTP Server**: Axum (v0.8.x) & Tower Service
- **Runtime**: Tokio
- **Serialization**: Buffa (pure-Rust, zero-copy Protobuf implementation) & Serde

---

## Local Execution Instructions

### 1. Requirements
Ensure you have the `buf` CLI tool installed and the required cargo codegen plugins on your `PATH`:
```bash
cargo install connectrpc-codegen protoc-gen-buffa protoc-gen-buffa-packaging
```

### 2. Code Generation
Whenever you modify your `.proto` file in the `proto/` directory, regenerate the Rust code:
```bash
buf generate proto
```

### 3. Running the Server
Start the Axum HTTP server (serving on port `8080`):
```bash
cargo run --bin server
```

### 4. Running the Client
In a separate terminal, execute the client binary to perform a test request:
```bash
cargo run --bin client
```

## License

<TBD> is released under the [MIT license](./LICENSE.md).

## Copyright

copyright:: (c) Copyright 2026 Conrad Taylor. All Rights Reserved.
