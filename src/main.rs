use ffi::build_witness;

#[cxx::bridge(namespace = "circom")]
mod ffi {
    // Shared structs with fields visible to both languages.
    // struct BlobMetadata {
    //     size: usize,
    //     tags: Vec<String>,
    // }

    // Rust types and signatures exposed to C++.
    extern "Rust" {}

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("rust-witness/circom/circom.hpp");

        fn build_witness();
    }
}

fn main() {
    build_witness();
}
