use std::env;
use std::path::Path;
use std::process::Command;
use std::{ffi::OsStr, fs};
use walkdir::WalkDir;

fn main() {
    Command::new("./build_w2c2.sh")
        .spawn()
        .expect("Failed to spawn w2c2 build")
        .wait()
        .expect("w2c2 build errored");
    let env_var = env::var("CIRCOM_WASM_DIR");
    let wasm_dir;
    // If the environment variable is not defined _and_ we are building
    // for tests we should use the local test vectors directory.
    // If we're not building for tests we just skip the transpilation
    if env_var.is_ok() {
        // the env variable is defined, parse as needed
        wasm_dir = env_var.unwrap();
        let wasm_dir_path = Path::new(&wasm_dir);
        if !Path::is_absolute(&wasm_dir_path) {
            panic!("CIRCOM_WASM_DIR must be an absolute path");
        }
        if !Path::is_dir(&wasm_dir_path) {
            panic!("CIRCOM_WASM_DIR is not a directory");
        }
        // paths = fs::read_dir(wasm_dir).unwrap();
    } else if env::var("PROFILE").unwrap() == "debug" {
        // the env var is not defined and we're building for debug
        // assume we're testing the rust-witness package itself
        // and build the test vectors
        println!("building from test-vectors");
        wasm_dir = String::from("./test-vectors");
        // paths = fs::read_dir("./test-vectors").unwrap();
    } else {
        // the env var is not defined and we're building for release
        // assume the package was included in another package but not used
        // and do nothing
        println!("CIRCOM_WASM_DIR is not defined, skipping witness build.");
        return;
    }
    let circuit_out_dir = "./circuits";
    let mut builder = cc::Build::new();
    // empty the handlers file
    let mut handler = "".to_string();
    builder
        .file("./circuits/globals.c")
        .file("./circuits/handlers.c")
        .flag("-I./w2c2/w2c2")
        .flag("-Wno-unused-label")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wnonull-character")
        .flag("-Wno-c2x-extensions");

    for entry in WalkDir::new(wasm_dir) {
        let e = entry.unwrap();
        let path = e.path();
        if path.is_dir() {
            continue;
        }
        let ext = path.extension().and_then(OsStr::to_str).unwrap();
        // Iterate over all wasm files and generate c source, then compile each source to
        // a static library that can be called from rust
        if ext != "wasm" {
            continue;
        }
        // with make source files with the same name as the wasm binary file
        let circuit_name = path.file_stem().unwrap();
        let circuit_name_compressed = circuit_name.to_str().unwrap().replace("_", "");
        handler.push_str(
            format!(
                "void {}_runtime__exceptionHandler(void*) {{ }}\n",
                circuit_name_compressed
            )
            .as_str(),
        );
        handler.push_str(
            format!(
                "void {}_runtime__printErrorMessage(void*) {{ }}\n",
                circuit_name_compressed
            )
            .as_str(),
        );
        let out = Path::new(&circuit_out_dir)
            .join(Path::new(path.file_name().unwrap()))
            .with_extension("c");
        if out.exists() {
            println!(
                "Source file already exists, overwriting: {}",
                &out.display()
            );
        }
        // first generate the c source
        Command::new("w2c2/build/w2c2/w2c2")
            .arg("-p")
            .arg("-m")
            .arg(path)
            .arg(out.clone())
            .spawn()
            .expect("Failed to spawn w2c2")
            .wait()
            .expect("w2c2 command errored");

        let contents = fs::read_to_string(out.clone()).unwrap();
        // make the data constants static to prevent duplicate symbol errors
        fs::write(
            out.clone(),
            contents.replace("const U8 d", "static const U8 d"),
        )
        .expect("Error modifying data symbols");

        builder.file(out.clone());
    }

    // write filename prefixed handler functions
    let handlers = Path::new(circuit_out_dir).join("handlers.c");
    fs::write(handlers, handler).expect("Error writing handler source");

    builder.compile("circuit");

    println!("cargo:rerun-if-changed=circuits/*");
}
