# Rust basic examples

A collection of various examples in Rust.

## Usage

```
cargo run --example cli_app
cargo run --example format_string
cargo run --example proc_macros
# etc.
```

## Running clippy with a different toolchain (eg. beta)

```
rustup toolchain install beta
cargo +beta clippy --all-targets
```
