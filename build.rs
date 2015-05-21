use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("go").args(&["build", "-buildmode=c-archive", "-o"])
        .arg(&format!("{}/libbuildmodes.a", out_dir))
        .arg(&"src/go/main.go")
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=buildmodes");
}
