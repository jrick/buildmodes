use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("go").args(&["build", "-buildmode=c-shared", "-o"])
        .arg(&format!("{}/libbuildmodes.so", out_dir))
        .arg(&"src/go/main.go")
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=buildmodes");
}
