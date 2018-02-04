extern crate cc;
use std::fs;
use std::process::Command;

fn main() {
    fs::create_dir_all("target/debug");
    fs::create_dir_all("target/release");
    
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .file("./src/shp2obj.cpp")
        .include("./src/")
        .compile("shp2obj");

    if cfg!(windows) {
        println!("build in windows:");
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=gdal_i");
    } else {
        println!("cargo:rustc-link-search=native=.");
    }
}