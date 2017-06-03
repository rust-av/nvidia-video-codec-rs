extern crate bindgen;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: bindgen::Builder, output: &str) {
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

fn common_builder() -> bindgen::Builder {
    bindgen::builder()
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

fn find_dir(default: &'static str, env_key: &'static str) -> PathBuf {
    match env::var_os(env_key) {
        Some(val) => PathBuf::from(&val),
        _ => PathBuf::from(default),
    }
}

fn main() {
    let cuda_include = find_dir("/opt/cuda/include", "CUDA_INCLUDE_PATH");
    let nvc_include = find_dir("/opt/nvidia-video-codec/include",
                               "NVIDIA_VIDEO_CODEC_INCLUDE_PATH");

    // TODO support windows
    println!("cargo:rustc-link-lib=dylib={}", "cuda");
    println!("cargo:rustc-link-lib=dylib={}", "nvcuvid");
    println!("cargo:rustc-link-lib=dylib={}", "nvidia-encode");


    let cuda_builder = common_builder()
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .header(cuda_include.join("cuda.h").to_string_lossy());

    // Manually fix the comment so rustdoc won't try to pick them
    format_write(cuda_builder, "src/cuda.rs");

    let cuvid_builder = common_builder()
        .clang_arg(format!("-I{}", nvc_include.to_string_lossy()))
        .clang_arg(format!("-I{}", cuda_include.to_string_lossy()))
        .header(nvc_include.join("nvcuvid.h").to_string_lossy());

    format_write(cuvid_builder, "src/cuvid.rs");

    let nvenc_builder = common_builder()
        .clang_arg(format!("-I{}", nvc_include.to_string_lossy()))
        .header(nvc_include.join("nvEncodeAPI.h").to_string_lossy());

    format_write(nvenc_builder, "src/nvenc.rs");
}
