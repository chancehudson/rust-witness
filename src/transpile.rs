use std::env;
use std::path::Path;
use std::process::Command;
use std::{ffi::OsStr, fs};
use walkdir::WalkDir;

// There are some files from this package that we need
// in packages that want to use this package.
// To pass these files we write them into the OUT_DIR
// from the build script. e.g. package a depends on rust-witness
// and calls transpile_wasm. The below files are writting into a/target/build
//
// TODO: figure out how to communicate between build scripts?

const W2C2_BUILD_SCRIPT: &str = include_str!("../build_w2c2.sh");
const GLOBALS_C: &str = include_str!("./globals.c");

pub fn transpile_wasm(wasmdir: String) {
    let w2c2_script_path =
        Path::new(env::var("OUT_DIR").unwrap().as_str()).join(Path::new("build_w2c2.sh"));
    fs::write(&w2c2_script_path, W2C2_BUILD_SCRIPT).expect("Failed to write build script");
    Command::new("sh")
        .arg(w2c2_script_path.to_str().unwrap())
        .spawn()
        .expect("Failed to spawn w2c2 build")
        .wait()
        .expect("w2c2 build errored");
    let globals_c_path = Path::new(&env::var("OUT_DIR").unwrap()).join(Path::new("globals.c"));
    fs::write(&globals_c_path, GLOBALS_C).expect("Failed to write globals.c");
    let w2c2_path = Path::new(env::var("OUT_DIR").unwrap().as_str()).join(Path::new("w2c2"));
    let w2c2_exec_path = w2c2_path.join(Path::new("build/w2c2/w2c2"));
    if !Path::is_dir(Path::new(wasmdir.as_str())) {
        panic!("wasmdir must be a directory");
    }

    let circuit_out_dir = env::var("OUT_DIR").unwrap();
    let mut builder = cc::Build::new();
    // empty the handlers file
    let mut handler = "".to_string();
    builder
        .file(globals_c_path.to_str().unwrap())
        .file(
            Path::new(circuit_out_dir.as_str())
                .join(Path::new("handlers.c"))
                .to_str()
                .unwrap(),
        )
        .flag(format!("-I{}", w2c2_path.join("w2c2").to_str().unwrap()).as_str())
        .flag("-Wno-unused-label")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-null-character")
        .flag("-Wno-c2x-extensions");

    for entry in WalkDir::new(wasmdir) {
        let e = entry.unwrap();
        let path = e.path();
        if path.is_dir() {
            continue;
        }
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
        // Iterate over all wasm files and generate c source, then compile each source to
        // a static library that can be called from rust
        if ext != "wasm" {
            continue;
        }
        // with make source files with the same name as the wasm binary file
        let circuit_name = path.file_stem().unwrap();
        let circuit_name_compressed = circuit_name
            .to_str()
            .unwrap()
            .replace("_", "")
            .replace("-", "");
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
        Command::new(w2c2_exec_path.to_str().unwrap())
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
    let handlers = Path::new(circuit_out_dir.as_str()).join("handlers.c");
    fs::write(handlers, handler).expect("Error writing handler source");

    builder.compile("circuit");
}
