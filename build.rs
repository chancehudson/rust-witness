use std::path::Path;
use std::process::Command;
use std::{ffi::OsStr, fs};

fn main() {
    let circuit_dir = "./circuits";
    let paths = fs::read_dir(circuit_dir).unwrap();
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
        .flag("-Wno-c2x-extensions");

    for entry in paths {
        let path = entry.unwrap().path();
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
        let out = path
            .clone()
            .with_file_name(path.clone().file_name().unwrap())
            .with_extension("c");
        if out.exists() {
            println!(
                "Source file already exists, overwriting: {}",
                out.clone().display()
            );
        }
        // first generate the c source
        Command::new("w2c2/build/w2c2/w2c2")
            .arg("-p")
            .arg("-m")
            .arg(path.clone())
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
    let handlers = Path::new(circuit_dir.clone()).join("handlers.c");
    fs::write(handlers, handler).expect("Error writing handler source");

    builder.compile("circuit");

    println!("cargo:rerun-if-changed=circuits/*");
}
