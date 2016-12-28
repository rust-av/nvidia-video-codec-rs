extern crate libbindgen;

// use std::env;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

fn format_write(builder: libbindgen::Builder, output: &str) {
    let s = builder.generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)
        .unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> libbindgen::Builder {
    libbindgen::builder()
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

fn main() {
    //    let out_dir = env::var("OUT_DIR").unwrap();
    // TODO make them params and try to figure them out as default
    let cuda_include = "/opt/cuda/include";
    let nvc_include = "/opt/nvidia-video-codec/";

    // TODO support windows
    println!("cargo:rustc-link-lib=dylib={}", "cuda");
    println!("cargo:rustc-link-lib=dylib={}", "nvcuvid");
    println!("cargo:rustc-link-lib=dylib={}", "nvidia-encode");


    let cuda_builder = common_builder()
        .clang_arg(format!("-I{}", cuda_include))
        .header(Path::new(cuda_include).join("cuda.h").to_str().unwrap());

    // Manually fix the comment so rustdoc won't try to pick them
    format_write(cuda_builder, "src/ffi/cuda.rs");

    let cuvid_builder = common_builder()
        .clang_arg(format!("-I{}", nvc_include))
        .clang_arg(format!("-I{}", cuda_include))
        .header(Path::new(nvc_include).join("nvcuvid.h").to_str().unwrap());

    println!("{:?}", cuvid_builder);

    format_write(cuvid_builder, "src/ffi/cuvid.rs");

    let nvenc_builder = common_builder()
        .clang_arg(format!("-I{}", nvc_include))
        .header(Path::new(nvc_include).join("nvEncodeAPI.h").to_str().unwrap());

    format_write(nvenc_builder, "src/ffi/nvenc.rs");
}

// let s = cuda_gen.generate()
// .unwrap()
// .to_string()
// .replace("**", "*");
//
// let mut file = OpenOptions::new()
// .write(true)
// .truncate(true)
// .create(true)
// .open("src/ffi/cuda.rs")
// .unwrap();
//
// let _ = file.write(s.as_bytes());
//
