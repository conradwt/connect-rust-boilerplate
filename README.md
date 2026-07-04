# Connect Rust Boilerplate Templates

This is a template repository configured for [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) to quickly spin up ConnectRPC services in Rust. It houses two distinct templates representing the two recommended code generation workflows in the Connect ecosystem.

## Scaffolding Options

1. **Checked In Code (Buf Generate)**: Uses local `buf` and `protoc-gen-connect-rust`/`protoc-gen-buffa` plugins to compile your Protocol Buffer schemas into checked-in source files.
2. **Dynamic Code (Build.rs)**: Uses Cargo build scripts and `connectrpc-build` to dynamically compile Protocol Buffer schemas at build time.

---

## How to Generate Projects

To use these templates, ensure you have `cargo-generate` installed:
```bash
cargo install cargo-generate
```

### 1. Generating from the Remote Git Repository
Once you push this boilerplate repository to your remote Git account, you can generate projects from it by passing the relative template path as a positional argument:

#### Option A: Checked In Code (Buf Generate)
```bash
cargo generate --git https://github.com/your-username/connect-rust-boilerplate.git templates/buf-generate \
  --name my-buf-service \
  --define proto_name=greet
```

#### Option B: Dynamic Code (Build.rs)
```bash
cargo generate --git https://github.com/your-username/connect-rust-boilerplate.git templates/build-rs \
  --name my-build-service \
  --define proto_name=greet
```

### 2. Generating from the Local Repository
You can test the templates locally by pointing the `--path` flag directly to the target subdirectory:

#### Option A: Checked In Code (Buf Generate)
```bash
cargo generate --path /Users/conradwt/projects.dir/connect-rust-boilerplate/templates/buf-generate \
  --name my-local-buf-service \
  --define proto_name=greet
```

#### Option B: Dynamic Code (Build.rs)
```bash
cargo generate --path /Users/conradwt/projects.dir/connect-rust-boilerplate/templates/build-rs \
  --name my-local-build-service \
  --define proto_name=greet
```

---

## Customizing the Protocol Buffer Name
You can customize the name of the generated `.proto` file (which dynamically renames the schema, package namespace, and service traits in the output) by passing the `--define proto_name=<your_custom_name>` flag during generation.
