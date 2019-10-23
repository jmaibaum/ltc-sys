use std::env;
use std::path::PathBuf;

fn main() {
    // Build libltc
    let src = [
        "vendor/src/ltc.c",
        "vendor/src/decoder.c",
        "vendor/src/encoder.c",
        "vendor/src/timecode.c",
    ];

    let mut builder = cc::Build::new();
    let build = builder.files(src.iter()).include("vendor/src");
    build.compile("ltc");

    let bindings = bindgen::Builder::default()
        .header("vendor/src/ltc.h")
        // FIXME: Wait for next bindgen release, see: https://github.com/rust-lang/rust-bindgen/issues/1647
        //.parse_callbacks(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
