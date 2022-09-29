fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    std::fs::copy("memory.x", out_dir.join("memory.x")).unwrap();
}
