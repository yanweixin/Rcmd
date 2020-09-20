fn main() {
    println!("cargo:rustc-link-search=src/clib/lib");
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/clib/library.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/clib/library.c")
        .compile("library");
}