# Protobuf Rust example

A very simple client/server application to try out [PROST!](https://github.com/tokio-rs/prost) for Protobuf support in Rust.

## Requirements

- Rust
- Protobuf compiler (protoc)

## Usage example

Start the server:
```bash
cargo run --bin server
```

Send one message to the server via the client:
```bash
cargo run --bin client localhost
```
