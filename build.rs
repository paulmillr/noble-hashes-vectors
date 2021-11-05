extern crate bindgen;
use std::{env, path::PathBuf};

fn build_k12() {
    println!("cargo:rustc-link-lib=K12");
    println!("cargo:rerun-if-changed=cfg/wrapper_k12.h");
    bindgen::Builder::default()
        .clang_arg("-Icfg/")
        .clang_arg("-IXKCP/lib/common/")
        .clang_arg("-IXKCP/lib/low/KeccakP-1600/ref-64bits/")
        .clang_arg("-IXKCP/lib/high/common/")
        .clang_arg("-IXKCP/lib/high/Keccak/")
        .header("cfg/wrapper_k12.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings_k12.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .static_flag(true)
        .define("KeccakP1600_noAssembly", "1")
        .include("cfg/")
        .include("XKCP/lib/common/")
        .include("XKCP/lib/low/KeccakP-1600/ref-64bits/")
        .include("XKCP/lib/high/common/")
        .include("XKCP/lib/high/Keccak/")
        .file("XKCP/lib/high/KangarooTwelve/KangarooTwelve.c")
        .compile("K12");
}

fn build_sp800_185() {
    println!("cargo:rustc-link-lib=SP800-185");
    println!("cargo:rerun-if-changed=cfg/wrapper_sp800_185.h");
    cc::Build::new()
        .static_flag(true)
        .define("KeccakP1600_noAssembly", "1")
        .include("cfg/")
        .include("XKCP/lib/common/")
        .include("XKCP/lib/low/KeccakP-1600/ref-64bits/")
        .include("XKCP/lib/high/common/")
        .include("XKCP/lib/high/Keccak/")
        .file("XKCP/lib/low/KeccakP-1600/ref-64bits/KeccakP-1600-reference.c")
        .file("XKCP/lib/high/Keccak/KeccakSponge.c")
        .file("XKCP/lib/high/Keccak/SP800-185/SP800-185.c")
        .compile("SP800-185");

    bindgen::Builder::default()
        .clang_arg("-Icfg/")
        .clang_arg("-IXKCP/lib/common/")
        .clang_arg("-IXKCP/lib/low/KeccakP-1600/ref-64bits/")
        .clang_arg("-IXKCP/lib/high/common/")
        .clang_arg("-IXKCP/lib/high/Keccak/")
        .header("cfg/wrapper_sp800_185.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings_sp800_185.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    // Separate builds since K12 and SP800-185 contains implementations of
    // rightEncode at same time
    build_sp800_185();
    build_k12();
}
