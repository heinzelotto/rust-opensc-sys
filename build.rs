extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system opensc
    // shared library.
    println!("cargo:rustc-link-lib=dylib=opensc");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let binding_builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // include OpenSC headers
        .clang_arg("-IOpenSC/src");

    // optional OpenSSL support
    #[cfg(feature = "enable_openssl")]
    let binding_builder = binding_builder.clang_arg("-DENABLE_OPENSSL");

    // optional Secure Messaging implementation
    #[cfg(feature = "enable_sm")]
    let binding_builder = binding_builder.clang_arg("-DENABLE_SM");

    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    let bindings = binding_builder.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
