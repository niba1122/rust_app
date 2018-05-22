# Rust App

## Required Environment

* Rust(nightly)
* cargo
* cargo web

## Build

```bash
cargo +nightly web build --target wasm32-unknown-unknown && \
cp target/wasm32-unknown-unknown/release/rust_app.js ./ && \
cp target/wasm32-unknown-unknown/release/rust_app.wasm ./
```
