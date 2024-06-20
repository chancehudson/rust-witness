# rust-witness

A crate to natively generate circom witnesses in Rust. This crate transpiles the wasm witness generator to C then provides macros to easily invoke the C functions.

## Setup

Clone the repo then run the following commands in the repo directory:

```sh
./build_w2c2.sh
cp test-vectors/* circuits/
cargo test
```

