# rust-witness

A crate to natively generate circom witnesses in Rust. This crate transpiles the wasm witness generator to C then provides macros to easily invoke the C functions.

## Usage

1. Add `rust-witness` as a dependency and a build dependency. Add `paste` as a dependency.
2. Write a `build.rs` that looks like the following:
```rust
use rust_witness::transpile::transpile_wasm;

fn main() {
    // This function will recursively search the target directory
    // for any files with the `wasm` extension and compile
    // them to C and link them
    transpile_wasm("my/path/to/wasm/");
}
```
3. Compute a witness like the following:
```rust
use rust_witness::witness;

// Use this macro to generate a function that can be
// used to build a witness for the target circuit
//
witness!(circuitname);
// The name should be the name of the wasm file all lowercase
// with all special characters removed
//
// e.g. 
// multiplier2 -> multiplier2
// keccak_256_256_main -> keccak256256main
// aadhaar-verifier -> aadhaarverifier
// 

fn build_proof() {
    let inputs: HashMap<String, Vec<BigInt>>;
    // The generated function will be the name of the circuit
    // followed by _witness
    let witness = circuitname_witness(inputs);
}
```


## Setup

Clone the repo then run the following command in the repo directory:

```sh
cargo test
```

## Speeding up builds

By default this package will build `w2c2` from source if the binary is not in your `$PATH`. This can be slow when developing and rebuilding frequently. To install `w2c2` to your cargo bin run the following:

```sh
sh ./install_w2c2.sh
```

`transpile_wasm` will automatically use the `w2c2` in your `$PATH` if possible.

## Toolchain Requirements

Using `rust-witness` in another crate requires Rust 1.81.0 to be installed in the environment. The toolchain can be installed with the following command:

```sh
rustup install 1.81.0
```

Without the correct toolchain, errors similar to the following may occur:
```sh 
error: linking with `cc` failed: exit status: 1
```