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
}
