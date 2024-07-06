// This is a hack to avoid nesting packages
// relevant imports are stored in this file and
// imported unhygienically
include!("./src/transpile.rs");

fn main() {
    if cfg!(test) {
        transpile_wasm(String::from("./tests"));
    }
}
