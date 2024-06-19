fn main() {
    cxx_build::bridge("src/main.rs")
        .include("depends/json/include")
        .include("depends/gmp/package/include")
        .object("circom/main.o")
        .object("circom/calcwit.o")
        .object("circom/stub_circuit.o")
        .object("circom/fr_raw_generic.o")
        .object("circom/fr_generic.o")
        .object("circom/fr.o")
        .file("src/witness.cpp")
        .std("c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/*.cpp");
    println!("cargo:rerun-if-changed=src/*.hpp");
    // link gmp
    println!("cargo:rustc-link-search=native=depends/gmp/package/lib");
    println!("cargo:rustc-link-lib=static=gmp");
}
