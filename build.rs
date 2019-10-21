use std::env;
use std::path::PathBuf;

fn main() {
    // Link the system ltc shared library
    println!("cargo:rustc-link-lib=ltc");

    // FIXME: Wait for next bindgen release, see: https://github.com/rust-lang/rust-bindgen/issues/1647
    // Invalidate built crate whenever `wrapper.h` changes
    //println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // FIXME: Wait for next bindgen release, see: https://github.com/rust-lang/rust-bindgen/issues/1647
        //.parse_callbacks(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
