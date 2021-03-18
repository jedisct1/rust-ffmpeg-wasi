use std::env;

pub fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=avfilter");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=swscale");
    println!("cargo:rustc-link-lib=static=c-wasm");
    println!("cargo:rustc-link-lib=static=c-builtins");
    println!("cargo:rustc-link-lib=static=vpx");
    println!("cargo:rustc-link-search=native={}/wasm-libs", src_dir);
}
