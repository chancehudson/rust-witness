// This is a hack to avoid nesting packages
// relevant imports are stored in this file and
// imported unhygienically
include!("./src/transpile.rs");

// We only want to link our test witness functions if we're compiling
// for the tests in rust-witness. Cargo/rust doesn't seem to offer
// a solution so we use a simple env variable in the test command
fn main() {
    let link_test_witness =
        std::env::var("RUST_WITNESS_LINK_TEST_WITNESS").unwrap_or("".to_string());
    if link_test_witness != "" {
        transpile_wasm(String::from("./tests"));
    }
}
