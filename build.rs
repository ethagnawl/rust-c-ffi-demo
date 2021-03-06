// Copied mostly verbatim from:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["src/lib/greet.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/greet.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["cvq", "libgreet.a", "greet.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=greet");
}
