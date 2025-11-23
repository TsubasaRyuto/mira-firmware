use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy memory.x to the output directory.
    fs::copy("memory.x", out_dir.join("memory.x")).unwrap();

    // Add the output directory to the linker search path.
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Rerun the build script if memory.x changes.
    println!("cargo:rerun-if-changed=memory.x");
}
