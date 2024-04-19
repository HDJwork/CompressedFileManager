use std::env;

fn main() {

    println!("cargo:warning=Build start!");
//    println!("build run!");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let mut _vendor_dir = "";
    match target_arch.as_str() {
        "x86_64" => _vendor_dir = "../ref",
        "x86" => _vendor_dir = "../ref",
        arch => panic!("x86 or x86_64 architecture needed ! (not {})", arch),
    }
    println!("cargo:rustc-link-search={}", _vendor_dir); // the "-L" flag  
}