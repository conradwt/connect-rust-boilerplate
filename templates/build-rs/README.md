# Connect Rust - Dynamic Code Template (Build.rs)

This template scaffolds a ConnectRPC service that compiles Protocol Buffer definitions dynamically at build time using a Cargo build script (`build.rs`).

## Tech Stack
- **Language**: Rust (Edition 2024)
- **Protocol**: ConnectRPC, gRPC, and gRPC-Web
- **HTTP Server**: Axum (v0.8.x) & Tower Service
- **Runtime**: Tokio
- **Serialization**: Buffa (pure-Rust, zero-copy Protobuf implementation) & Serde
- **Build compiler**: `connectrpc-build`

---

## Local Execution Instructions

### 1. Requirements
Ensure you have the Protocol Buffer compiler (`protoc`) installed on your system.

### 2. Running the Server
Cargo runs the `build.rs` script automatically during compilation, meaning code generation happens seamlessly on build. Start the Axum HTTP server (serving on port `8080`):
```bash
cargo run --bin server
```

### 3. Running the Client
In a separate terminal, execute the client binary to perform a test request:
```bash
cargo run --bin client
```

## License

Connect Rust - Dynamic Code Template is released under the [MIT license](./LICENSE.md).

## Copyright

copyright:: (c) Copyright 2026 Conrad Taylor. All Rights Reserved.
